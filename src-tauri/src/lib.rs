mod commands;
mod config;
mod serial;
mod state;
mod utils;
mod record;

use anyhow::{Error, Result};
use state::AppState;
use tauri::{generate_context, generate_handler, Manager};
use tokio::sync::Mutex;

use crate::commands::{
    command_connect_port,
    command_get_data_grid,
    command_get_serialports,
    command_reset_port,
    command_append_record,
    command_last_record,
    command_pop_record,
    command_reset_records,
    command_save_records,
    command_records_len,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_devtools::try_init()?)
        .invoke_handler(generate_handler![
            command_connect_port,
            command_get_data_grid,
            command_get_serialports,
            command_reset_port,
            command_append_record,
            command_last_record,
            command_pop_record,
            command_reset_records,
            command_save_records,
            command_records_len,
        ])
        .run(generate_context!())
        .map_err(|err| Error::new(err).context("error while running tauri application"))
}
