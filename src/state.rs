// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::state::State;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Machine {
    Planned,
    Provisioning,
    Initialized,
    Ready,
    Failed,
}

impl Machine {
    pub fn can_transition_to(&self, target: &Machine) -> bool {
        self.valid_transitions().contains(target)
    }

    pub fn valid_transitions(&self) -> Vec<Machine> {
        match self {
            Machine::Planned => vec![Machine::Provisioning],
            Machine::Provisioning => vec![Machine::Initialized, Machine::Failed],
            Machine::Initialized => vec![Machine::Ready, Machine::Failed],
            Machine::Ready => vec![],
            Machine::Failed => vec![],
        }
    }

    pub fn transition_to(self, target: Machine) -> Result<Machine, State> {
        if self.can_transition_to(&target) {
            Ok(target)
        } else {
            Err(State::InvalidTransition {
                current: self,
                attempted: target,
                valid_transitions: self
                    .valid_transitions()
                    .iter()
                    .map(|state| state.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            })
        }
    }
}

impl Display for Machine {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Machine::Planned => write!(formatter, "planned"),
            Machine::Provisioning => write!(formatter, "provisioning"),
            Machine::Initialized => write!(formatter, "initialized"),
            Machine::Ready => write!(formatter, "ready"),
            Machine::Failed => write!(formatter, "failed"),
        }
    }
}

impl FromStr for Machine {
    type Err = State;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "planned" => Ok(Machine::Planned),
            "provisioning" => Ok(Machine::Provisioning),
            "initialized" => Ok(Machine::Initialized),
            "ready" => Ok(Machine::Ready),
            "failed" => Ok(Machine::Failed),
            _ => Err(State::UnknownState {
                value: value.to_string(),
            }),
        }
    }
}
