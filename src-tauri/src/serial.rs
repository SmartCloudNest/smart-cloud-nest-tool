use anyhow::{bail, Error, Result};
use serialport::SerialPort;
use std::time::Duration;

pub struct Port<'a, const SERIAL_BUFFER_LENGTH: usize> {
    serialport: Box<dyn SerialPort>,
    serial_buffer: Box<[u8; SERIAL_BUFFER_LENGTH]>,
    data_buffer: Vec<u8>,
    delimiter: &'a [u8],
    max_data_buffer_length: usize,
    target_data_field_length: usize,
}

impl<'a, const SERIAL_BUFFER_LENGTH: usize> Port<'a, SERIAL_BUFFER_LENGTH> {
    pub fn build(
        name: String,
        baud_rate: u32,
        delimiter: &'a [u8],
        max_data_buffer_length: usize,
        target_data_field_length: usize,
        timeout: Duration,
    ) -> Result<Self> {
        if delimiter.is_empty() {
            bail!("delimiter cannot be empty");
        }
        if delimiter.len() > SERIAL_BUFFER_LENGTH {
            bail!("SERIAL_BUFFER_LENGTH must >= delimiter length");
        }
        if SERIAL_BUFFER_LENGTH < target_data_field_length {
            bail!("SERIAL_BUFFER_LENGTH must be >= target_data_field");
        }
        if max_data_buffer_length < target_data_field_length {
            bail!("max_data_buffer_length must be >= target_data_field");
        }
        let serialport = serialport::new(name.to_owned(), baud_rate)
            .timeout(timeout)
            .open()
            .map_err(|err| {
                Error::new(err).context(format!(
                    "Failed to open port {}, baud rate {}",
                    name, baud_rate
                ))
            })?;
        let serial_buffer = Box::new([0; SERIAL_BUFFER_LENGTH]);
        let data_buffer = Vec::default();
        let port = Self {
            serialport,
            serial_buffer,
            data_buffer,
            delimiter,
            max_data_buffer_length,
            target_data_field_length,
        };
        return Ok(port);
    }

    fn pull_serial_data(&mut self) -> Result<&[u8]> {
        let data_length = self.serialport.read(self.serial_buffer.as_mut_slice())?;
        let full_data = &self.serial_buffer[..data_length];
        Ok(full_data)
    }

    fn find_delimiter_position(&self) -> Option<usize> {
        self.data_buffer
            .windows(self.delimiter.len())
            .position(|sub_slice| sub_slice.eq(self.delimiter))
    }

    pub fn pend_data_field(&mut self) -> Result<Vec<u8>> {
        loop {
            if let Some(delimiter_position) = self.find_delimiter_position() {
                let tail = &self.data_buffer[delimiter_position + self.delimiter.len()..];
                if delimiter_position != self.target_data_field_length {
                    self.data_buffer = tail.to_vec();
                    continue;
                }
                let head = self.data_buffer[..delimiter_position].to_vec();
                self.data_buffer = tail.to_vec();
                return Ok(head);
            }
            let mut fresh_serial_data = self.pull_serial_data()?.to_vec();
            self.data_buffer.append(&mut fresh_serial_data);
            if self.data_buffer.len() > self.max_data_buffer_length {
                let truncation = self.data_buffer.len() - self.max_data_buffer_length;
                self.data_buffer = self.data_buffer[truncation..].to_vec();
            }
        }
    }
}

impl<'a, const SERIAL_BUFFER_LENGTH: usize> Iterator for Port<'a, SERIAL_BUFFER_LENGTH> {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pend_data_field().ok()
    }
}
