// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::Validation,
    laboratories::{Configuration, Laboratory, Machine},
    logger::Logger,
    providers::{Drive, MachineDefinition, Provider as ProviderTrait},
};
use miette::{Diagnostic, Result};
use serde_json::{Value, json};
use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

const DEFAULT_DEPLOY_TIMEOUT_SECONDS: u64 = 30 * 60;

pub fn execute(path: PathBuf) -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|error| miette::miette!("Failed to start deployment runtime: {}", error))?;

    runtime.block_on(execute_async(path))
}

async fn execute_async(path: PathBuf) -> Result<()> {
    Logger::info("Checking file access");

    if !path.exists() {
        return Err(Validation::ConfigurationNotFound {
            path: path.display().to_string(),
        }
        .into());
    }

    if path.is_dir() {
        return Err(Validation::ConfigurationPathIsDirectory {
            path: path.display().to_string(),
        }
        .into());
    }

    let source_code = fs::read_to_string(&path).map_err(|error| {
        if error.kind() == io::ErrorKind::InvalidData {
            Validation::ConfigurationInvalidUtf8 {
                path: path.display().to_string(),
            }
        } else {
            Validation::ConfigurationReadError {
                path: path.display().to_string(),
                source: error,
            }
        }
    })?;

    let source_name = path.display().to_string();

    Logger::info("Parsing configuration");

    let configuration = Configuration::parse(&source_name, &source_code)?;

    let parent = path.parent().unwrap_or(path.as_path()).to_owned();

    Logger::info("Validating configuration");

    configuration.validate(&source_name, &source_code, &parent)?;

    Logger::info("Resolving provider");

    let provider = configuration.laboratory.provider.into_provider()?;

    Logger::info(&format!(
        "Resolved provider boundary: {}",
        provider.identifier()
    ));

    let deployment_start = Instant::now();
    let deployment_started_at = current_unix_seconds();
    let correlation_identifier =
        correlation_identifier(configuration.laboratory.name.value.as_str());
    let deployment_directory = deployment_directory(&correlation_identifier)?;
    let summary_path = deployment_summary_path(&correlation_identifier)?;
    let deploy_timeout = Duration::from_secs(DEFAULT_DEPLOY_TIMEOUT_SECONDS);
    let interruption_signal = Arc::new(AtomicBool::new(false));

    spawn_interrupt_listener(Arc::clone(&interruption_signal));

    let mut summary = DeploymentSummary::new(
        correlation_identifier,
        configuration.laboratory.name.value.as_str().to_owned(),
        provider.identifier().to_owned(),
        source_name.clone(),
        deployment_started_at,
        DEFAULT_DEPLOY_TIMEOUT_SECONDS,
    );

    Logger::info("Verifying provider availability");

    if let Err(error) = provider.verify_availability().await {
        summary.status = DeploymentStatus::Failed;
        summary.error = Some(error.to_string());
        summary.add_not_attempted_machines(&configuration.machines.value);
        summary.finished_at_unix_seconds = Some(current_unix_seconds());
        summary.elapsed_seconds = deployment_start.elapsed().as_secs_f64();
        persist_summary(&summary_path, &summary)?;

        return Err(error.into());
    }

    let mut deployment_failure: Option<(DeploymentStatus, miette::Report)> = None;

    for (index, machine) in configuration.machines.value.iter().enumerate() {
        if interruption_signal.load(Ordering::SeqCst) {
            deployment_failure = Some((
                DeploymentStatus::Interrupted,
                DeploymentError::Interrupted {
                    elapsed_seconds: deployment_start.elapsed().as_secs_f64(),
                    remediation: "Allow the current machine operation to settle, then use `malina diagnose` to inspect the interruption point before retrying deployment.".to_owned(),
                }
                .into(),
            ));
            summary.interruption = Some(InterruptionRecord::new(
                "signal",
                "between-machines",
                Some(machine.identifier.value.as_str().to_owned()),
                "Ctrl+C received while deployment was progressing".to_owned(),
            ));
            break;
        }

        if deployment_start.elapsed() > deploy_timeout {
            deployment_failure = Some((
                DeploymentStatus::TimedOut,
                DeploymentError::TimedOut {
                    elapsed_seconds: deployment_start.elapsed().as_secs_f64(),
                    remediation: "Review the provider, guest-init artifacts, and machine boot logs, then rerun deployment after confirming the host is healthy.".to_owned(),
                }
                .into(),
            ));
            summary.interruption = Some(InterruptionRecord::new(
                "timeout",
                "between-machines",
                Some(machine.identifier.value.as_str().to_owned()),
                "Deployment exceeded the configured timeout".to_owned(),
            ));
            break;
        }

        Logger::info(&format!(
            "Provisioning machine {}/{}: {}",
            index + 1,
            configuration.machines.value.len(),
            machine.identifier.value.as_str()
        ));

        match deploy_machine(
            provider.as_ref(),
            &configuration.laboratory,
            machine,
            &parent,
            &deployment_directory,
        )
        .await
        {
            Ok(machine_summary) => summary.machines.push(machine_summary),
            Err((machine_summary, error)) => {
                summary.machines.push(machine_summary);
                summary.status = DeploymentStatus::Failed;
                summary.error = Some(error.to_string());
                summary.add_not_attempted_machines(&configuration.machines.value[index + 1..]);
                summary.finished_at_unix_seconds = Some(current_unix_seconds());
                summary.elapsed_seconds = deployment_start.elapsed().as_secs_f64();
                persist_summary(&summary_path, &summary)?;

                return Err(error);
            }
        }
    }

    if deployment_failure.is_none() {
        if interruption_signal.load(Ordering::SeqCst) {
            deployment_failure = Some((
                DeploymentStatus::Interrupted,
                DeploymentError::Interrupted {
                    elapsed_seconds: deployment_start.elapsed().as_secs_f64(),
                    remediation: "Use `malina diagnose` to inspect the saved deployment summary, then rerun deployment when the operator is ready.".to_owned(),
                }
                .into(),
            ));
            summary.interruption = Some(InterruptionRecord::new(
                "signal",
                "after-machine",
                summary
                    .machines
                    .last()
                    .map(|machine| machine.identifier.clone()),
                "Ctrl+C received after a machine phase completed".to_owned(),
            ));
        } else if deployment_start.elapsed() > deploy_timeout {
            deployment_failure = Some((
                DeploymentStatus::TimedOut,
                DeploymentError::TimedOut {
                    elapsed_seconds: deployment_start.elapsed().as_secs_f64(),
                    remediation: "Review the provider, guest-init artifacts, and machine boot logs, then rerun deployment after confirming the host is healthy.".to_owned(),
                }
                .into(),
            ));
            summary.interruption = Some(InterruptionRecord::new(
                "timeout",
                "after-machine",
                summary
                    .machines
                    .last()
                    .map(|machine| machine.identifier.clone()),
                "Deployment exceeded the configured timeout".to_owned(),
            ));
        }
    }

    summary.finished_at_unix_seconds = Some(current_unix_seconds());
    summary.elapsed_seconds = deployment_start.elapsed().as_secs_f64();

    if let Some((status, error)) = deployment_failure {
        summary.status = status;
        if summary.error.is_none() {
            summary.error = Some(error.to_string());
        }
        summary.add_not_attempted_machines(&configuration.machines.value[summary.machines.len()..]);
        persist_summary(&summary_path, &summary)?;

        return Err(error);
    }

    summary.status = DeploymentStatus::Success;
    persist_summary(&summary_path, &summary)?;

    Logger::print(&format!(
        "Deployment completed for {} (summary: {})",
        summary.laboratory_identifier,
        summary_path.display()
    ));

    Ok(())
}

async fn deploy_machine(
    provider: &dyn ProviderTrait,
    laboratory: &Laboratory,
    machine: &Machine,
    parent: &Path,
    deployment_directory: &Path,
) -> Result<MachineSummary, (MachineSummary, miette::Report)> {
    let machine_identifier = machine.identifier.value.as_str().to_owned();
    let mut summary = MachineSummary::new(&machine_identifier, machine.name.clone());

    let image_path =
        match prepare_machine_artifacts(machine, parent, deployment_directory, &machine_identifier)
        {
            Ok(path) => path,
            Err(error) => {
                summary.fail("prepare_machine_artifacts", error.to_string(), "failed");
                return Err((summary, error));
            }
        };

    Logger::info(&format!("Creating machine {}", machine_identifier));

    if let Err(error) = provider
        .create_machine(MachineDefinition {
            laboratory: laboratory.clone(),
            machine: machine.clone(),
            image: image_path,
        })
        .await
    {
        summary.fail("create_machine", error.to_string(), "failed");
        return Err((summary, error.into()));
    }

    summary.complete_phase("create_machine");
    summary.final_state = "provisioning".to_owned();

    Logger::info(&format!("Generating guest-init for {}", machine_identifier));

    let seed_bytes = match machine
        .operating_system
        .seeder()
        .generate_iso(machine, parent)
    {
        Ok(bytes) => bytes,
        Err(error) => {
            summary.fail("generate_guest_init", error.to_string(), "failed");
            return Err((summary, error.into()));
        }
    };

    let guest_init_path = deployment_directory.join(format!("{}.iso", machine_identifier));

    if let Err(error) = fs::write(&guest_init_path, &seed_bytes) {
        let report = miette::miette!(
            "Failed to write guest-init artifact for {} at {}: {}",
            machine_identifier,
            guest_init_path.display(),
            error
        );
        summary.fail(
            "generate_guest_init",
            format!("failed to write guest-init artifact: {}", error),
            "failed",
        );
        return Err((summary, report));
    }

    summary.complete_phase("generate_guest_init");
    summary.final_state = "initialized".to_owned();

    Logger::info(&format!("Attaching guest-init for {}", machine_identifier));

    if let Err(error) = provider
        .inject_drive(Drive {
            identifier: machine.identifier.value.clone(),
            path: guest_init_path,
        })
        .await
    {
        summary.fail("attach_guest_init", error.to_string(), "failed");
        return Err((summary, error.into()));
    }

    summary.complete_phase("attach_guest_init");

    Logger::info(&format!("Starting machine {}", machine_identifier));

    if let Err(error) = provider
        .start_machine(machine.identifier.value.clone())
        .await
    {
        summary.fail("start_machine", error.to_string(), "failed");
        return Err((summary, error.into()));
    }

    summary.complete_phase("start_machine");
    summary.outcome = "completed".to_owned();
    summary.final_state = "ready".to_owned();

    Ok(summary)
}

fn prepare_machine_artifacts(
    machine: &Machine,
    parent: &Path,
    deployment_directory: &Path,
    machine_identifier: &str,
) -> Result<PathBuf> {
    let source_image = if machine.operating_system.image_path().is_absolute() {
        machine.operating_system.image_path().to_path_buf()
    } else {
        parent.join(machine.operating_system.image_path())
    };

    if !source_image.exists() {
        return Err(miette::miette!(
            "Source image for {} was not found at {}",
            machine_identifier,
            source_image.display()
        ));
    }

    let image_directory = deployment_directory.join("images");
    fs::create_dir_all(&image_directory).map_err(|error| {
        miette::miette!(
            "Failed to create copied image directory {}: {}",
            image_directory.display(),
            error
        )
    })?;

    let copied_image_path = image_directory.join(format!("{}.qcow2", machine_identifier));
    fs::copy(&source_image, &copied_image_path).map_err(|error| {
        miette::miette!(
            "Failed to copy machine image from {} to {}: {}",
            source_image.display(),
            copied_image_path.display(),
            error
        )
    })?;

    Logger::info(&format!(
        "Prepared machine artifacts: image={}",
        copied_image_path.display()
    ));

    Ok(copied_image_path)
}

fn persist_summary(path: &Path, summary: &DeploymentSummary) -> Result<()> {
    let json = build_summary_json(summary);
    let content = serde_json::to_string_pretty(&json)
        .map_err(|error| miette::miette!("Failed to encode deployment summary: {}", error))?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            miette::miette!(
                "Failed to create deployment summary directory {}: {}",
                parent.display(),
                error
            )
        })?;
    }

    fs::write(path, content).map_err(|error| {
        miette::miette!(
            "Failed to write deployment summary {}: {}",
            path.display(),
            error
        )
    })
}

fn build_summary_json(summary: &DeploymentSummary) -> Value {
    json!({
        "correlation_identifier": &summary.correlation_identifier,
        "laboratory_identifier": &summary.laboratory_identifier,
        "provider_identifier": &summary.provider_identifier,
        "configuration_path": &summary.configuration_path,
        "status": summary.status.as_str(),
        "started_at_unix_seconds": summary.started_at_unix_seconds,
        "finished_at_unix_seconds": summary.finished_at_unix_seconds,
        "elapsed_seconds": summary.elapsed_seconds,
        "timeout_seconds": summary.timeout_seconds,
        "error": &summary.error,
        "interruption": summary.interruption.as_ref().map(InterruptionRecord::to_json),
        "machines": summary.machines.iter().map(MachineSummary::to_json).collect::<Vec<_>>(),
    })
}

fn correlation_identifier(laboratory_identifier: &str) -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    format!(
        "deploy-{}-{:x}-{:x}-{:x}",
        laboratory_identifier,
        duration.as_secs(),
        duration.subsec_nanos(),
        process::id()
    )
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn deployment_directory(correlation_identifier: &str) -> Result<PathBuf> {
    let base = deployment_root_directory();
    let directory = base.join(correlation_identifier);

    fs::create_dir_all(&directory).map_err(|error| {
        miette::miette!(
            "Failed to create deployment artifact directory {}: {}",
            directory.display(),
            error
        )
    })?;

    Ok(directory)
}

fn deployment_summary_path(correlation_identifier: &str) -> Result<PathBuf> {
    Ok(deployment_root_directory().join(format!("{}.json", correlation_identifier)))
}

fn deployment_root_directory() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".malina")
        .join("deployments")
}

fn spawn_interrupt_listener(flag: Arc<AtomicBool>) {
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        flag.store(true, Ordering::SeqCst);
    });
}

#[derive(Debug, Clone)]
struct DeploymentSummary {
    correlation_identifier: String,
    laboratory_identifier: String,
    provider_identifier: String,
    configuration_path: String,
    status: DeploymentStatus,
    started_at_unix_seconds: u64,
    finished_at_unix_seconds: Option<u64>,
    elapsed_seconds: f64,
    timeout_seconds: u64,
    error: Option<String>,
    interruption: Option<InterruptionRecord>,
    machines: Vec<MachineSummary>,
}

impl DeploymentSummary {
    fn new(
        correlation_identifier: String,
        laboratory_identifier: String,
        provider_identifier: String,
        configuration_path: String,
        started_at_unix_seconds: u64,
        timeout_seconds: u64,
    ) -> Self {
        Self {
            correlation_identifier,
            laboratory_identifier,
            provider_identifier,
            configuration_path,
            status: DeploymentStatus::InProgress,
            started_at_unix_seconds,
            finished_at_unix_seconds: None,
            elapsed_seconds: 0.0,
            timeout_seconds,
            error: None,
            interruption: None,
            machines: Vec::new(),
        }
    }

    fn add_not_attempted_machines(&mut self, machines: &[Machine]) {
        self.machines
            .extend(machines.iter().map(MachineSummary::not_attempted));
    }
}

#[derive(Debug, Clone)]
struct MachineSummary {
    identifier: String,
    name: String,
    final_state: String,
    outcome: String,
    error: Option<String>,
    phases: Vec<PhaseRecord>,
}

impl MachineSummary {
    fn new(identifier: &str, name: String) -> Self {
        Self {
            identifier: identifier.to_owned(),
            name,
            final_state: "planned".to_owned(),
            outcome: "in-progress".to_owned(),
            error: None,
            phases: Vec::new(),
        }
    }

    fn not_attempted(machine: &Machine) -> Self {
        Self {
            identifier: machine.identifier.value.as_str().to_owned(),
            name: machine.name.clone(),
            final_state: "planned".to_owned(),
            outcome: "not_attempted".to_owned(),
            error: None,
            phases: Vec::new(),
        }
    }

    fn complete_phase(&mut self, name: &'static str) {
        self.phases.push(PhaseRecord {
            name: name.to_owned(),
            status: PhaseStatus::Completed.as_str().to_owned(),
            error: None,
        });
    }

    fn fail(&mut self, name: &'static str, error: String, final_state: &'static str) {
        self.phases.push(PhaseRecord {
            name: name.to_owned(),
            status: PhaseStatus::Failed.as_str().to_owned(),
            error: Some(error.clone()),
        });
        self.final_state = final_state.to_owned();
        self.outcome = "failed".to_owned();
        self.error = Some(error);
    }

    fn to_json(&self) -> Value {
        json!({
            "identifier": &self.identifier,
            "name": &self.name,
            "final_state": &self.final_state,
            "outcome": &self.outcome,
            "error": &self.error,
            "phases": self.phases.iter().map(PhaseRecord::to_json).collect::<Vec<_>>(),
        })
    }
}

#[derive(Debug, Clone)]
struct PhaseRecord {
    name: String,
    status: String,
    error: Option<String>,
}

impl PhaseRecord {
    fn to_json(&self) -> Value {
        json!({
            "name": &self.name,
            "status": &self.status,
            "error": &self.error,
        })
    }
}

#[derive(Debug, Clone)]
struct InterruptionRecord {
    kind: String,
    phase: String,
    machine_identifier: Option<String>,
    message: String,
}

impl InterruptionRecord {
    fn new(
        kind: &'static str,
        phase: &'static str,
        machine_identifier: Option<String>,
        message: String,
    ) -> Self {
        Self {
            kind: kind.to_owned(),
            phase: phase.to_owned(),
            machine_identifier,
            message,
        }
    }

    fn to_json(&self) -> Value {
        json!({
            "kind": &self.kind,
            "phase": &self.phase,
            "machine_identifier": &self.machine_identifier,
            "message": &self.message,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeploymentStatus {
    InProgress,
    Success,
    Failed,
    Interrupted,
    TimedOut,
}

impl DeploymentStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::InProgress => "in_progress",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Interrupted => "interrupted",
            Self::TimedOut => "timed_out",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PhaseStatus {
    Completed,
    Failed,
}

impl PhaseStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
enum DeploymentError {
    #[diagnostic(help("{remediation}"))]
    #[error("Deployment timed out after {elapsed_seconds:.3} seconds")]
    TimedOut {
        elapsed_seconds: f64,
        remediation: String,
    },

    #[diagnostic(help("{remediation}"))]
    #[error("Deployment interrupted after {elapsed_seconds:.3} seconds")]
    Interrupted {
        elapsed_seconds: f64,
        remediation: String,
    },
}
