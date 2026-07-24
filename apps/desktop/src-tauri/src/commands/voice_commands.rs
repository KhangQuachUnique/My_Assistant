use tauri::{AppHandle, State};

use crate::services::voice_process_service::VoiceProcessService;

#[tauri::command]
pub async fn start_voice_worker(
    app: AppHandle,
    voice_service: State<'_, VoiceProcessService>,
) -> Result<(), String> {
    voice_service.start(app)
}

#[tauri::command]
pub async fn stop_voice_worker(
    voice_service: State<'_, VoiceProcessService>,
) -> Result<(), String> {
    voice_service.stop()
}
