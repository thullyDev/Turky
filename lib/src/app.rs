use crate::commands;
use crate::services;
use crate::state::AppState;


pub fn run() {
    let display_service =
        services::display_service::DisplayService::new();
    let state = AppState::new(display_service);
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state);

    commands::register(builder)
        .run(tauri::generate_context!())
        .expect("error while running turky");
}