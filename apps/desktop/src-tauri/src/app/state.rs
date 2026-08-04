use std::sync::{Mutex, MutexGuard};

use crate::{
    app::status::ApplicationStatus,
    shared::{config::AppConfig, error::AppError},
};

#[derive(Debug)]
pub struct AppState {
    config: AppConfig,
    status: Mutex<ApplicationStatus>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            status: Mutex::new(ApplicationStatus::Created),
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn status(&self) -> Result<ApplicationStatus, AppError> {
        let status = self.lock_status()?;

        Ok(*status)
    }

    pub(crate) fn transition(
        &self,
        expected: ApplicationStatus,
        next: ApplicationStatus,
    ) -> Result<(), AppError> {
        let mut current = self.lock_status()?;

        if *current != expected {
            return Err(AppError::Lifecycle(format!(
                "cannot transition from {:?} to {:?}; expected current status {:?}",
                *current, next, expected
            )));
        }

        *current = next;

        Ok(())
    }

    pub(crate) fn begin_shutdown(&self) -> Result<bool, AppError> {
        let mut current = self.lock_status()?;

        match *current {
            ApplicationStatus::Stopping | ApplicationStatus::Stopped => Ok(false),

            ApplicationStatus::Ready | ApplicationStatus::Running => {
                *current = ApplicationStatus::Stopping;
                Ok(true)
            }

            status => Err(AppError::Lifecycle(format!(
                "cannot begin shutdown while application is in {status:?}"
            ))),
        }
    }

    fn lock_status(&self) -> Result<MutexGuard<'_, ApplicationStatus>, AppError> {
        self.status
            .lock()
            .map_err(|_| AppError::Internal(String::from("application status lock is poisoned")))
    }
}
