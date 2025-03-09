use anyhow::{Error, Result};
use serialport::available_ports;
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    config::{
        BAUD_RATE, DELIMITER, MAX_DATA_BUFFER_LENGTH, SERIAL_BUFFER_LENGTH,
        TARGET_DATA_FIELD_LENGTH,
    },
    serial::Port,
};

#[derive(Default)]
pub struct AppState {
    port: Option<Port<'static, SERIAL_BUFFER_LENGTH>>,
}

impl AppState {
    pub fn connect_port(&mut self, name: String) -> Result<()> {
        let fresh_port = Port::build(
            name,
            BAUD_RATE,
            &DELIMITER,
            MAX_DATA_BUFFER_LENGTH,
            TARGET_DATA_FIELD_LENGTH,
        )?;
        let _ = self.port.replace(fresh_port);
        Ok(())
    }

    pub fn reset_port(&mut self) {
        let _ = self.port.take();
    }

    pub fn pend_data_field(&mut self) -> Result<Vec<u8>> {
        self.port
            .as_mut()
            .ok_or(Error::msg("Port instant isn't initialized"))?
            .pend_data_field()
    }
}

pub fn get_serialports() -> Vec<String> {
    available_ports()
        .unwrap_or(Vec::default())
        .into_iter()
        .map(|port| port.port_name)
        .collect()
}

pub type TauriAppState<'a> = State<'a, Mutex<AppState>>;
