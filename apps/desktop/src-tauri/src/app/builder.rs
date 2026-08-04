use tauri::Manager;

use crate::app::{lifecycle, state::AppState};

pub fn run() {
    let app_state = lifecycle::initialize().expect("failed to initialize application");

    let app = tauri::Builder::default()
        .manage(app_state)
        .build(tauri::generate_context!())
        .expect("failed to build Tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::Ready => {
            let app_state = app_handle.state::<AppState>();

            if let Err(error) = lifecycle::start(&app_state) {
                eprintln!("failed to start application lifecycle: {error}");
            }
        }

        tauri::RunEvent::ExitRequested { .. } => {
            let app_state = app_handle.state::<AppState>();

            if let Err(error) = lifecycle::shutdown(&app_state) {
                eprintln!("failed to shut down application: {error}");
            }
        }

        _ => {}
    });
}
