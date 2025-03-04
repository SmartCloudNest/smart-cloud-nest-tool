use tauri::{command, State};

use crate::serial::{get_serialports, get_sub_seq, AppState};

#[command]
pub async fn command_set_serialport(
    state: State<'_, AppState>,
    name: String,
) -> Result<(), String> {
    state
        .set_serialport(name)
        .await
        .or_else(|err| Err(err.to_string()))
}

#[command]
pub async fn command_get_data_seq(state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    let full_seq = state
        .get_serial_data()
        .await
        .or_else(|err| Err(err.to_string()))?;
    get_sub_seq(full_seq)
        .await
        .or_else(|err| Err(err.to_string()))
}

#[command]
pub async fn command_get_serialports() -> Result<Vec<String>, String> {
    get_serialports().await.or_else(|err| Err(err.to_string()))
}
