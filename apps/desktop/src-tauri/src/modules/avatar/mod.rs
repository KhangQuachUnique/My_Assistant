//! Avatar business module.
//!
//! This module will own avatar lifecycle rules without depending
//! directly on Tauri or Windows APIs.

mod error;
mod outcome;
mod process;
mod service;

pub use error::AvatarError;
pub use outcome::{AvatarRestartOutcome, AvatarStartOutcome, AvatarStopOutcome};
pub use process::AvatarProcessRunner;
pub use service::AvatarService;

use crate::modules::runtime::{ModuleLifecycleStatus, ModuleState, RuntimeModule};

#[derive(Debug)]
pub struct AvatarModule {
    state: ModuleState,
    service: AvatarService,
}

impl AvatarModule {
    pub fn new(service: AvatarService) -> Self {
        Self {
            state: ModuleState::new(),
            service,
        }
    }

    #[allow(dead_code)]
    pub fn restart(&self) -> Result<AvatarRestartOutcome, AvatarError> {
        match self.status() {
            ModuleLifecycleStatus::Starting | ModuleLifecycleStatus::Stopping => {
                return Err(AvatarError::Busy);
            }
            _ => {}
        }

        self.state.set_status(ModuleLifecycleStatus::Stopping);
        self.state.set_status(ModuleLifecycleStatus::Starting);

        match self.service.restart() {
            Ok(outcome) => {
                self.state.set_status(ModuleLifecycleStatus::Running);
                Ok(outcome)
            }
            Err(error) => {
                self.state.set_status(ModuleLifecycleStatus::Failed);
                Err(error)
            }
        }
    }
}

impl RuntimeModule for AvatarModule {
    type Error = AvatarError;
    type StartOutcome = AvatarStartOutcome;
    type StopOutcome = AvatarStopOutcome;

    fn name(&self) -> &'static str {
        "avatar"
    }

    fn start(&self) -> Result<Self::StartOutcome, Self::Error> {
        match self.status() {
            ModuleLifecycleStatus::Running => return Ok(AvatarStartOutcome::AlreadyRunning),
            ModuleLifecycleStatus::Starting | ModuleLifecycleStatus::Stopping => {
                return Err(AvatarError::Busy);
            }
            _ => {}
        }

        self.state.set_status(ModuleLifecycleStatus::Starting);

        match self.service.start() {
            Ok(outcome) => {
                self.state.set_status(ModuleLifecycleStatus::Running);
                Ok(outcome)
            }
            Err(error) => {
                self.state.set_status(ModuleLifecycleStatus::Failed);
                Err(error)
            }
        }
    }

    fn stop(&self) -> Result<Self::StopOutcome, Self::Error> {
        match self.status() {
            ModuleLifecycleStatus::Idle | ModuleLifecycleStatus::Stopped => {
                return Ok(AvatarStopOutcome::AlreadyStopped);
            }
            ModuleLifecycleStatus::Starting | ModuleLifecycleStatus::Stopping => {
                return Err(AvatarError::Busy);
            }
            _ => {}
        }

        self.state.set_status(ModuleLifecycleStatus::Stopping);

        match self.service.stop() {
            Ok(outcome) => {
                self.state.set_status(ModuleLifecycleStatus::Stopped);
                Ok(outcome)
            }
            Err(error) => {
                self.state.set_status(ModuleLifecycleStatus::Failed);
                Err(error)
            }
        }
    }

    fn status(&self) -> ModuleLifecycleStatus {
        self.state.status()
    }
}

impl Default for AvatarModule {
    fn default() -> Self {
        Self::new(AvatarService::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_starts_from_idle() {
        let avatar = AvatarModule::default();

        assert_eq!(avatar.status(), ModuleLifecycleStatus::Idle);

        let outcome = avatar.start().expect("avatar should start");

        assert_eq!(outcome, AvatarStartOutcome::Started);
        assert_eq!(avatar.status(), ModuleLifecycleStatus::Running);
    }

    #[test]
    fn avatar_start_is_idempotent_while_running() {
        let avatar = AvatarModule::default();

        avatar.start().expect("first start should succeed");

        let outcome = avatar.start().expect("second start should succeed");

        assert_eq!(outcome, AvatarStartOutcome::AlreadyRunning);
        assert_eq!(avatar.status(), ModuleLifecycleStatus::Running);
    }

    #[test]
    fn avatar_stop_moves_running_module_to_stopped() {
        let avatar = AvatarModule::default();

        avatar.start().expect("start should succeed");
        let outcome = avatar.stop().expect("stop should succeed");

        assert_eq!(outcome, AvatarStopOutcome::Stopped);
        assert_eq!(avatar.status(), ModuleLifecycleStatus::Stopped);
    }

    #[test]
    fn avatar_restart_starts_idle_module() {
        let avatar = AvatarModule::default();

        let outcome = avatar.restart().expect("restart should start avatar");

        assert_eq!(outcome, AvatarRestartOutcome::Started);
        assert_eq!(avatar.status(), ModuleLifecycleStatus::Running);
    }

    #[test]
    fn avatar_restart_keeps_running_module_running() {
        let avatar = AvatarModule::default();

        avatar.start().expect("start should succeed");
        let outcome = avatar.restart().expect("restart should succeed");

        assert_eq!(outcome, AvatarRestartOutcome::Restarted);
        assert_eq!(avatar.status(), ModuleLifecycleStatus::Running);
    }
}
