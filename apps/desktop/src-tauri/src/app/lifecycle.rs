use crate::{
    app::{
        state::{AppModules, AppState},
        status::ApplicationStatus,
    },
    modules::{
        avatar::{AvatarModule, AvatarService},
        runtime::RuntimeModule,
    },
    platform::windows::avatar::WindowsAvatarProcessRunner,
    shared::{config::AppConfig, error::AppError},
};

pub fn initialize() -> Result<AppState, AppError> {
    let config = AppConfig::default();

    initialize_with_modules(config, build_app_modules())
}

fn initialize_with_modules(config: AppConfig, modules: AppModules) -> Result<AppState, AppError> {
    let app_state = AppState::new(config, modules);

    app_state.transition(ApplicationStatus::Created, ApplicationStatus::Initializing)?;

    validate_config(app_state.config())?;

    app_state.transition(ApplicationStatus::Initializing, ApplicationStatus::Ready)?;

    Ok(app_state)
}

fn build_app_modules() -> AppModules {
    AppModules::new(build_avatar_module())
}

fn build_avatar_module() -> AvatarModule {
    AvatarModule::new(AvatarService::with_process(Box::new(
        WindowsAvatarProcessRunner::new(),
    )))
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

    fn initialize_for_test() -> AppState {
        initialize_with_modules(
            AppConfig::default(),
            AppModules::new(AvatarModule::new(AvatarService::new())),
        )
        .expect("application initialization should succeed")
    }

    #[test]
    fn initialize_moves_application_to_ready() {
        let app_state = initialize_for_test();

        assert_eq!(
            app_state.status().expect("status should be readable"),
            ApplicationStatus::Ready
        );
    }

    #[test]
    fn start_moves_ready_application_to_running() {
        let app_state = initialize_for_test();

        start(&app_state).expect("application start should succeed");

        assert_eq!(
            app_state.status().expect("status should be readable"),
            ApplicationStatus::Running
        );
        assert_eq!(app_state.avatar().status(), ModuleLifecycleStatus::Running);
    }

    #[test]
    fn shutdown_moves_running_application_to_stopped() {
        let app_state = initialize_for_test();

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
        let app_state = initialize_for_test();

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
        let app_state = initialize_for_test();

        start(&app_state).expect("first start should succeed");

        let result = start(&app_state);

        assert!(matches!(result, Err(AppError::Lifecycle(_))));
    }
}
