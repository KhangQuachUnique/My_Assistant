use serde::Serialize;
use tauri::State;

use crate::{
    app::state::AppState,
    modules::{
        avatar::{AvatarError, AvatarRestartOutcome, AvatarStartOutcome, AvatarStopOutcome},
        runtime::{ModuleLifecycleStatus, RuntimeModule},
    },
};

pub fn handlers<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool {
    tauri::generate_handler![avatar_start, avatar_stop, avatar_restart, avatar_status]
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCommandResponse {
    status: AvatarLifecycleStatus,
    outcome: Option<AvatarLifecycleOutcome>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCommandError {
    code: AvatarCommandErrorCode,
    message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum AvatarCommandErrorCode {
    ExeNotFound,
    ProcessStartFailed,
    ProcessStopFailed,
    ProcessStatusUnavailable,
    Busy,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum AvatarLifecycleOutcome {
    Started,
    AlreadyRunning,
    Stopped,
    AlreadyStopped,
    Restarted,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum AvatarLifecycleStatus {
    Idle,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[tauri::command]
pub fn avatar_start(
    app_state: State<'_, AppState>,
) -> Result<AvatarCommandResponse, AvatarCommandError> {
    let avatar = app_state.avatar();
    let outcome = avatar.start().map_err(AvatarCommandError::from)?;

    Ok(AvatarCommandResponse::with_outcome(
        avatar.status(),
        AvatarLifecycleOutcome::from(outcome),
    ))
}

#[tauri::command]
pub fn avatar_stop(
    app_state: State<'_, AppState>,
) -> Result<AvatarCommandResponse, AvatarCommandError> {
    let avatar = app_state.avatar();
    let outcome = avatar.stop().map_err(AvatarCommandError::from)?;

    Ok(AvatarCommandResponse::with_outcome(
        avatar.status(),
        AvatarLifecycleOutcome::from(outcome),
    ))
}

#[tauri::command]
pub fn avatar_restart(
    app_state: State<'_, AppState>,
) -> Result<AvatarCommandResponse, AvatarCommandError> {
    let avatar = app_state.avatar();
    let outcome = avatar.restart().map_err(AvatarCommandError::from)?;

    Ok(AvatarCommandResponse::with_outcome(
        avatar.status(),
        AvatarLifecycleOutcome::from(outcome),
    ))
}

#[tauri::command]
pub fn avatar_status(app_state: State<'_, AppState>) -> AvatarCommandResponse {
    AvatarCommandResponse::without_outcome(app_state.avatar().status())
}

impl AvatarCommandResponse {
    fn with_outcome(
        status: ModuleLifecycleStatus,
        outcome: AvatarLifecycleOutcome,
    ) -> AvatarCommandResponse {
        AvatarCommandResponse {
            status: AvatarLifecycleStatus::from(status),
            outcome: Some(outcome),
        }
    }

    fn without_outcome(status: ModuleLifecycleStatus) -> AvatarCommandResponse {
        AvatarCommandResponse {
            status: AvatarLifecycleStatus::from(status),
            outcome: None,
        }
    }
}

impl From<AvatarError> for AvatarCommandError {
    fn from(error: AvatarError) -> Self {
        let code = match error {
            AvatarError::ExeNotFound(_) => AvatarCommandErrorCode::ExeNotFound,
            AvatarError::ProcessStartFailed(_) => AvatarCommandErrorCode::ProcessStartFailed,
            AvatarError::ProcessStopFailed(_) => AvatarCommandErrorCode::ProcessStopFailed,
            AvatarError::ProcessStatusUnavailable(_) => {
                AvatarCommandErrorCode::ProcessStatusUnavailable
            }
            AvatarError::Busy => AvatarCommandErrorCode::Busy,
        };

        Self {
            code,
            message: error.to_string(),
        }
    }
}

impl From<AvatarStartOutcome> for AvatarLifecycleOutcome {
    fn from(outcome: AvatarStartOutcome) -> Self {
        match outcome {
            AvatarStartOutcome::Started => Self::Started,
            AvatarStartOutcome::AlreadyRunning => Self::AlreadyRunning,
        }
    }
}

impl From<AvatarStopOutcome> for AvatarLifecycleOutcome {
    fn from(outcome: AvatarStopOutcome) -> Self {
        match outcome {
            AvatarStopOutcome::Stopped => Self::Stopped,
            AvatarStopOutcome::AlreadyStopped => Self::AlreadyStopped,
        }
    }
}

impl From<AvatarRestartOutcome> for AvatarLifecycleOutcome {
    fn from(outcome: AvatarRestartOutcome) -> Self {
        match outcome {
            AvatarRestartOutcome::Restarted => Self::Restarted,
            AvatarRestartOutcome::Started => Self::Started,
        }
    }
}

impl From<ModuleLifecycleStatus> for AvatarLifecycleStatus {
    fn from(status: ModuleLifecycleStatus) -> Self {
        match status {
            ModuleLifecycleStatus::Idle => Self::Idle,
            ModuleLifecycleStatus::Starting => Self::Starting,
            ModuleLifecycleStatus::Running => Self::Running,
            ModuleLifecycleStatus::Stopping => Self::Stopping,
            ModuleLifecycleStatus::Stopped => Self::Stopped,
            ModuleLifecycleStatus::Failed => Self::Failed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_response_serializes_for_ipc() {
        let response = AvatarCommandResponse::without_outcome(ModuleLifecycleStatus::Running);
        let json = serde_json::to_value(response).expect("response should serialize");

        assert_eq!(json["status"], "running");
        assert!(json["outcome"].is_null());
    }

    #[test]
    fn start_response_serializes_outcome_for_ipc() {
        let response = AvatarCommandResponse::with_outcome(
            ModuleLifecycleStatus::Running,
            AvatarLifecycleOutcome::Started,
        );
        let json = serde_json::to_value(response).expect("response should serialize");

        assert_eq!(json["status"], "running");
        assert_eq!(json["outcome"], "started");
    }

    #[test]
    fn avatar_error_serializes_code_and_message_for_ipc() {
        let error = AvatarCommandError::from(AvatarError::Busy);
        let json = serde_json::to_value(error).expect("error should serialize");

        assert_eq!(json["code"], "busy");
        assert_eq!(json["message"], "avatar module is busy");
    }
}
