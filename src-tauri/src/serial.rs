use std::time::Duration;

use crate::config::{
    BAUD_RATE, DATA_FIELD_LENGTH, DILIMITER, ROW_LENGTH, SERIAL_LENGTH, SUB_GRID_COL_LENGTH,
    SUB_GRID_ROW_LENGTH,
};
use anyhow::{Error, Result};
use serialport::{available_ports, SerialPort};
use tokio::{sync::Mutex, task::spawn_blocking};

pub struct AppState {
    serial: Mutex<Option<Box<dyn SerialPort>>>,
    serial_buffer: Mutex<[u8; SERIAL_LENGTH]>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            serial: Mutex::new(None),
            serial_buffer: Mutex::new([0; SERIAL_LENGTH]),
        }
    }
}

impl AppState {
    pub async fn set_serialport(&self, name: String) -> Result<()> {
        let port = serialport::new(name, BAUD_RATE)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|err| Error::new(err))?;
        self.serial.lock().await.replace(port);
        Ok(())
    }

    pub async fn get_serial_data(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::default();
        loop {
            self.read_serialport().await?;
            let mut serial_data = self.serial_buffer.lock().await.to_vec();
            buffer.append(&mut serial_data);
            let dilimiter_idx = buffer
                .windows(DILIMITER.len())
                .position(|sub_seq| DILIMITER.eq(sub_seq));
            if let Some(dilimiter_idx) = dilimiter_idx {
                let head = &buffer[..dilimiter_idx];
                let tail = &buffer[dilimiter_idx + DILIMITER.len()..];
                if head.len() != DATA_FIELD_LENGTH {
                    buffer = tail.to_owned();
                    continue;
                }
                let result = head.to_owned();
                return Ok(result);
            } else {
                continue;
            }
        }
    }

    async fn read_serialport(&self) -> Result<()> {
        if let Some(port) = self.serial.lock().await.as_mut() {
            port.read(self.serial_buffer.lock().await.as_mut_slice())
                .map_err(|err| Error::new(err))?;
            return Ok(());
        }
        Err(Error::msg("Serial port not selected!"))
    }
}

pub async fn get_serialports() -> Result<Vec<String>> {
    spawn_blocking(|| {
        let result = available_ports()
            .map_err(|err| Error::new(err))?
            .into_iter()
            .map(|serial| serial.port_name)
            .collect();
        Ok(result)
    })
    .await
    .map_err(|err| Error::new(err))?
}

pub async fn get_sub_seq(seq: Vec<u8>) -> Result<Vec<u8>> {
    spawn_blocking(move || {
        seq.chunks(ROW_LENGTH)
            .take(SUB_GRID_COL_LENGTH)
            .map(|row| row[0..SUB_GRID_ROW_LENGTH].to_vec())
            .flatten()
            .collect()
    })
    .await
    .map_err(|err| Error::new(err))
}
