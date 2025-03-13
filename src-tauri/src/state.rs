use std::path::PathBuf;
use anyhow::{anyhow, Result};
use serialport::available_ports;

use crate::{
    config::{
        BAUD_RATE, DELIMITER, MAX_DATA_BUFFER_LENGTH, SERIAL_BUFFER_LENGTH, SERIAL_TIMEOUT,
        TARGET_DATA_FIELD_LENGTH,
    }, record::{CsvRecord, CsvSheet}, serial::Port
};

#[derive(Default)]
pub struct AppState {
    port: Option<Port<'static, SERIAL_BUFFER_LENGTH>>,
    record: CsvSheet,
}

impl AppState {
    pub fn connect_port(&mut self, name: String) -> Result<()> {
        let fresh_port = Port::build(
            name,
            BAUD_RATE,
            &DELIMITER,
            MAX_DATA_BUFFER_LENGTH,
            TARGET_DATA_FIELD_LENGTH,
            SERIAL_TIMEOUT,
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
            .ok_or(anyhow!("Port instant isn't initialized"))?
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

impl AppState {
    pub fn append_record(&mut self, tag: String, data: String) {
        self.record.append(tag, data);
    }

    pub fn last_record(&self) -> Option<CsvRecord> {
        self.record.last()
    }

    pub fn pop_record(&mut self) -> Option<CsvRecord> {
        self.record.pop()
    }

    pub fn save_records(&self, path: PathBuf) -> Result<()> {
        self.record.save(path)
    }

    pub fn records_len(&self) -> usize {
        self.record.col_len()
    }

    pub fn reset_records(&mut self) {
        self.record.reset();
    }
}
