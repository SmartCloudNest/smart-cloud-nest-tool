mod commands;
mod config;
mod serial;
mod state;
mod utils;

use anyhow::{Error, Result};
use state::AppState;
use tauri::Manager;
use tokio::sync::Mutex;

use crate::commands::{
    command_connect_port, command_get_data_seq, command_get_serialports, command_reset_port,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command_get_serialports,
            command_reset_port,
            command_connect_port,
            command_get_data_seq,
        ])
        .run(tauri::generate_context!())
        .map_err(|err| Error::new(err))
}
