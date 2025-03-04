mod commands;
mod config;
mod serial;

use commands::{command_get_data_seq, command_get_serialports, command_set_serialport};
use serial::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command_get_data_seq,
            command_set_serialport,
            command_get_serialports,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
