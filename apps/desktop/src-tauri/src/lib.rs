mod commands;
mod services;

use commands::{
    tool_commands::run_tool,
    voice_commands::{start_voice_worker, stop_voice_worker},
};
use services::voice_process_service::VoiceProcessService;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(VoiceProcessService::new())
        .invoke_handler(tauri::generate_handler![
            run_tool,
            start_voice_worker,
            stop_voice_worker
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
