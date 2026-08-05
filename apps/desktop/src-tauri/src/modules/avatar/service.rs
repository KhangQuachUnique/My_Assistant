use std::fmt::{Debug, Formatter};

use super::{
    process::{AvatarProcessRunner, InMemoryAvatarProcessRunner},
    AvatarError, AvatarRestartOutcome, AvatarStartOutcome, AvatarStopOutcome,
};

pub struct AvatarService {
    process: Box<dyn AvatarProcessRunner>,
}

impl AvatarService {
    pub fn new() -> Self {
        Self::with_process(Box::new(InMemoryAvatarProcessRunner::new()))
    }

    pub fn with_process(process: Box<dyn AvatarProcessRunner>) -> Self {
        Self { process }
    }

    pub fn start(&self) -> Result<AvatarStartOutcome, AvatarError> {
        if self.process.is_running()? {
            return Ok(AvatarStartOutcome::AlreadyRunning);
        }

        self.process.start()?;
        Ok(AvatarStartOutcome::Started)
    }

    pub fn stop(&self) -> Result<AvatarStopOutcome, AvatarError> {
        if !self.process.is_running()? {
            return Ok(AvatarStopOutcome::AlreadyStopped);
        }

        self.process.stop()?;
        Ok(AvatarStopOutcome::Stopped)
    }

    #[allow(dead_code)]
    pub fn restart(&self) -> Result<AvatarRestartOutcome, AvatarError> {
        let was_running = self.process.is_running()?;

        if was_running {
            self.process.stop()?;
        }

        self.process.start()?;

        if was_running {
            Ok(AvatarRestartOutcome::Restarted)
        } else {
            Ok(AvatarRestartOutcome::Started)
        }
    }
}

impl Default for AvatarService {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for AvatarService {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AvatarService")
            .field("process", &"AvatarProcessRunner")
            .finish()
    }
}
