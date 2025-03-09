use tauri::command;

use crate::{
    state::{get_serialports, TauriAppState},
    utils::extract_sub_data_seq,
};

#[command]
pub async fn command_connect_port(state: TauriAppState<'_>, name: String) -> Result<(), String> {
    state
        .lock()
        .await
        .connect_port(name)
        .map_err(|err| err.to_string())
}

#[command]
pub async fn command_get_data_seq(state: TauriAppState<'_>) -> Result<Vec<u8>, String> {
    let full_data_seq = state
        .lock()
        .await
        .pend_data_field()
        .map_err(|err| err.to_string())?;
    let final_data_seq = extract_sub_data_seq(full_data_seq);
    Ok(final_data_seq)
}

#[command]
pub async fn command_reset_port(state: TauriAppState<'_>) -> Result<(), ()> {
    state.lock().await.reset_port();
    Ok(())
}

#[command]
pub async fn command_get_serialports() -> Vec<String> {
    get_serialports()
}
