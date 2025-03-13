use std::path::PathBuf;
use anyhow::Result;
use chrono::Utc;
use csv::Writer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CsvRecord {
    time: i64,
    tag: String,
    data: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CsvSheet {
    col: Vec<CsvRecord>
}

impl CsvSheet {
    pub fn append(&mut self, tag: String, data: String) {
        let record = CsvRecord {
            time: Utc::now().timestamp(),
            tag,
            data
        };
        self.col.push(record);
    }

    pub fn last(&self) -> Option<CsvRecord> {
        self.col.last().cloned()
    }

    pub fn pop(&mut self) -> Option<CsvRecord> {
        self.col.pop()
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        let mut writer = Writer::from_path(path)?;
        for record in &self.col {
            writer.serialize(record)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn col_len(&self) -> usize {
        self.col.len()
    }

    pub fn reset(&mut self) {
        self.col.clear();
    }
}