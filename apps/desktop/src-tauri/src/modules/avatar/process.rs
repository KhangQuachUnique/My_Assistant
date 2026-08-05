use std::sync::Mutex;

use super::AvatarError;

pub trait AvatarProcessRunner: Send + Sync {
    fn start(&self) -> Result<(), AvatarError>;

    fn stop(&self) -> Result<(), AvatarError>;

    fn is_running(&self) -> Result<bool, AvatarError>;
}

#[derive(Debug, Default)]
pub struct InMemoryAvatarProcessRunner {
    running: Mutex<bool>,
}

impl InMemoryAvatarProcessRunner {
    pub fn new() -> Self {
        Self {
            running: Mutex::new(false),
        }
    }

    fn set_running(&self, next: bool) -> Result<(), AvatarError> {
        let mut running = self
            .running
            .lock()
            .map_err(|_| AvatarError::ProcessStatusUnavailable(String::from("lock poisoned")))?;

        *running = next;
        Ok(())
    }
}

impl AvatarProcessRunner for InMemoryAvatarProcessRunner {
    fn start(&self) -> Result<(), AvatarError> {
        self.set_running(true)
    }

    fn stop(&self) -> Result<(), AvatarError> {
        self.set_running(false)
    }

    fn is_running(&self) -> Result<bool, AvatarError> {
        self.running
            .lock()
            .map(|running| *running)
            .map_err(|_| AvatarError::ProcessStatusUnavailable(String::from("lock poisoned")))
    }
}
