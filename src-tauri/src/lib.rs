mod commands;
mod config;
mod serial;

use anyhow::{Error, Result};
use commands::{command_get_data_seq, command_get_serialports, command_set_serialport, command_reset_serialport};
use serial::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()>{
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
            command_reset_serialport,
        ])
        .run(tauri::generate_context!())
        .map_err(|err| Error::new(err))
}
