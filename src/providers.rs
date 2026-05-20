// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod kvm;
mod libvirt;

pub use crate::providers::kvm::Kvm;
use crate::{
    errors,
    laboratories::{Identifier, Laboratory, Machine},
    state,
};
use std::{boxed::Box, path::PathBuf, pin::Pin};

pub type ProviderFuture<T> =
    Pin<Box<dyn std::future::Future<Output = Result<T, errors::Provider>> + Send>>;

#[derive(Debug, Clone)]
pub struct MachineDefinition {
    pub laboratory: Laboratory,
    pub machine: Machine,
    pub image: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Drive {
    pub identifier: Identifier,
    pub path: PathBuf,
}

pub trait Provider: Send + Sync {
    fn identifier(&self) -> &'static str;
    fn verify_availability(&self) -> ProviderFuture<()>;
    fn create_machine(&self, definition: MachineDefinition) -> ProviderFuture<()>;
    fn start_machine(&self, identifier: Identifier) -> ProviderFuture<()>;
    fn stop_machine(&self, identifier: Identifier, force: bool) -> ProviderFuture<()>;
    fn destroy_machine(&self, identifier: Identifier) -> ProviderFuture<()>;
    fn query_machine_state(&self, identifier: Identifier) -> ProviderFuture<state::Machine>;
    fn inject_drive(&self, drive: Drive) -> ProviderFuture<()>;
}
