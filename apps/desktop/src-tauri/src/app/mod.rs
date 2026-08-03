mod state;

pub use state::AppState;

pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .manage(app_state)
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
