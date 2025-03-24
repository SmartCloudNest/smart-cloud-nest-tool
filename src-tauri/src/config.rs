use std::time::Duration;

pub const SERIAL_BUFFER_LENGTH: usize = 2048;
pub const BAUD_RATE: u32 = 1_000_000;
pub const DELIMITER: [u8; 4] = [0xAA, 0x55, 0x03, 0x99];
pub const MAX_DATA_BUFFER_LENGTH: usize = 4096;
pub const TARGET_DATA_FIELD_LENGTH: usize = 1024;
pub const SERIAL_TIMEOUT: Duration = Duration::from_millis(130);
