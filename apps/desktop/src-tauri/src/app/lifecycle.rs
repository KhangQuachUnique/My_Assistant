use crate::{
    app::{state::AppState, status::ApplicationStatus},
    modules::runtime::RuntimeModule,
    shared::{config::AppConfig, error::AppError},
};

pub fn initialize() -> Result<AppState, AppError> {
    let config = AppConfig::default();
    let app_state = AppState::new(config);

    app_state.transition(ApplicationStatus::Created, ApplicationStatus::Initializing)?;

    validate_config(app_state.config())?;

    app_state.transition(ApplicationStatus::Initializing, ApplicationStatus::Ready)?;

    Ok(app_state)
}

pub fn start(app_state: &AppState) -> Result<(), AppError> {
    app_state.transition(ApplicationStatus::Ready, ApplicationStatus::Running)?;
    let avatar = app_state.avatar();

    avatar
        .start()
        .map_err(|error| AppError::Initialization(format!("{}: {error}", avatar.name())))?;

    debug_assert_eq!(
        app_state.status()?,
        ApplicationStatus::Running,
        "application should be running after startup"
    );

    println!("application is ready");
    Ok(())
}

pub fn shutdown(app_state: &AppState) -> Result<(), AppError> {
    let should_cleanup = app_state.begin_shutdown()?;

    if !should_cleanup {
        return Ok(());
    }

    println!("application is shutting down");

    cleanup(app_state)?;

    app_state.transition(ApplicationStatus::Stopping, ApplicationStatus::Stopped)?;

    println!("application has stopped");

    Ok(())
}

fn validate_config(config: &AppConfig) -> Result<(), AppError> {
    if config.application_name.trim().is_empty() {
        return Err(AppError::Configuration(String::from(
            "application name must not be empty",
        )));
    }

    Ok(())
}

fn cleanup(app_state: &AppState) -> Result<(), AppError> {
    let avatar = app_state.avatar();

    avatar
        .stop()
        .map_err(|error| AppError::Lifecycle(format!("{}: {error}", avatar.name())))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app::status::ApplicationStatus, modules::runtime::ModuleLifecycleStatus};

    #[test]
    fn initialize_moves_application_to_ready() {
        let app_state = initialize().expect("application initialization should succeed");

        assert_eq!(
            app_state.status().expect("status should be readable"),
            ApplicationStatus::Ready
        );
    }

    #[test]
    fn start_moves_ready_application_to_running() {
        let app_state = initialize().expect("application initialization should succeed");

        start(&app_state).expect("application start should succeed");

        assert_eq!(
            app_state.status().expect("status should be readable"),
            ApplicationStatus::Running
        );
        assert_eq!(app_state.avatar().status(), ModuleLifecycleStatus::Running);
    }

    #[test]
    fn shutdown_moves_running_application_to_stopped() {
        let app_state = initialize().expect("application initialization should succeed");

        start(&app_state).expect("application start should succeed");
        shutdown(&app_state).expect("application shutdown should succeed");

        assert_eq!(
            app_state.status().expect("status should be readable"),
            ApplicationStatus::Stopped
        );
        assert_eq!(app_state.avatar().status(), ModuleLifecycleStatus::Stopped);
    }

    #[test]
    fn shutdown_is_idempotent() {
        let app_state = initialize().expect("application initialization should succeed");

        start(&app_state).expect("application start should succeed");

        shutdown(&app_state).expect("first shutdown should succeed");
        shutdown(&app_state).expect("second shutdown should also succeed");

        assert_eq!(
            app_state.status().expect("status should be readable"),
            ApplicationStatus::Stopped
        );
    }

    #[test]
    fn application_cannot_start_twice() {
        let app_state = initialize().expect("application initialization should succeed");

        start(&app_state).expect("first start should succeed");

        let result = start(&app_state);

        assert!(matches!(result, Err(AppError::Lifecycle(_))));
    }
}
