use std::path::PathBuf;
use tauri::{command, State};
use tokio::sync::Mutex;
use tracing::debug;

use crate::{
    record::CsvRecord, state::{
        get_serialports, AppState
    }, utils::extract_final_grid
};

pub type TauriAppState<'a> = State<'a, Mutex<AppState>>;

#[command]
pub async fn command_connect_port(state: TauriAppState<'_>, name: String) -> Result<(), String> {
    state
        .lock()
        .await
        .connect_port(name)
        .map_err(|err| err.to_string())
}

#[command]
pub async fn command_get_data_grid(state: TauriAppState<'_>) -> Result<Vec<Vec<u8>>, String> {
    let full_data_seq = state.lock().await.pend_data_field().map_err(|err| {
        let err_text = err.to_string();
        debug!(err_text);
        err_text
    })?;
    let final_data_grid = extract_final_grid(full_data_seq);
    Ok(final_data_grid)
}

#[command]
pub async fn command_reset_port(state: TauriAppState<'_>) -> Result<(), String> {
    state
        .lock()
        .await
        .reset_port();
    Ok(())
}

#[command]
pub async fn command_get_serialports() -> Vec<String> {
    get_serialports()
}

#[command]
pub async fn command_append_record(state: TauriAppState<'_>, time: i64, tag: String, data: String) -> Result<(), String> {
    state
        .lock()
        .await
        .append_record(time, tag, data);
    Ok(())
}

#[command]
pub async fn command_last_record(state: TauriAppState<'_>) -> Result<Option<CsvRecord>, String> {
    let last = state
        .lock()
        .await
        .last_record();
    Ok(last)
}

#[command]
pub async fn command_pop_record(state: TauriAppState<'_>) -> Result<Option<CsvRecord>, String> {
    let poped = state
        .lock()
        .await
        .pop_record();
    Ok(poped)
}

#[command]
pub async fn command_save_records(state: TauriAppState<'_>, path: PathBuf) -> Result<(), String> {
    state.lock().await.save_records(path).map_err(|err| err.context("Failed to save record!").to_string())
}

#[command]
pub async fn command_records_len(state: TauriAppState<'_>) -> Result<usize, String> {
    let length = state
        .lock()
        .await
        .records_len();
    Ok(length)
}

#[command]
pub async fn command_reset_records(state: TauriAppState<'_>) -> Result<(), String> {
    state
        .lock()
        .await
        .reset_records();
    Ok(())
}