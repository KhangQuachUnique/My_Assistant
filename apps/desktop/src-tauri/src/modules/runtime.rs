use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleLifecycleStatus {
    Idle,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug)]
pub struct ModuleState {
    status: Mutex<ModuleLifecycleStatus>,
}

impl ModuleState {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(ModuleLifecycleStatus::Idle),
        }
    }

    pub fn status(&self) -> ModuleLifecycleStatus {
        self.lock_status()
            .map(|status| *status)
            .unwrap_or(ModuleLifecycleStatus::Failed)
    }

    pub fn set_status(&self, next: ModuleLifecycleStatus) {
        if let Ok(mut status) = self.lock_status() {
            *status = next;
        }
    }

    fn lock_status(&self) -> Result<MutexGuard<'_, ModuleLifecycleStatus>, ()> {
        self.status.lock().map_err(|_| ())
    }
}

impl Default for ModuleState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait RuntimeModule {
    type Error;
    type StartOutcome;
    type StopOutcome;

    fn name(&self) -> &'static str;

    fn start(&self) -> Result<Self::StartOutcome, Self::Error>;

    fn stop(&self) -> Result<Self::StopOutcome, Self::Error>;

    fn status(&self) -> ModuleLifecycleStatus;
}
