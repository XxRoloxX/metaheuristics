use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
pub struct CSVLogger<T> {
    output_filename: String,
    headers: Vec<String>,
    entries: Vec<T>,
}

pub trait PersistableLogger<T> {
    fn log(&mut self, entry: T);
    fn flush(&mut self) -> Result<u64>;
}

impl<T> CSVLogger<T> {
    pub fn new(output_filename: String, headers: Vec<String>) -> Self {
        CSVLogger {
            output_filename,
            entries: Vec::new(),
            headers,
        }
    }
}

#[derive(Debug, Default)]
pub struct CSVEntry {
    columns: Vec<String>,
}

impl CSVEntry {
    pub fn to_row(&self) -> String {
        self.columns.join("\t")
    }
    pub fn new() -> Self {
        CSVEntry {
            columns: Vec::new(),
        }
    }
}

impl From<Vec<String>> for CSVEntry {
    fn from(value: Vec<String>) -> Self {
        CSVEntry { columns: value }
    }
}

impl From<&CSVEntry> for String {
    fn from(value: &CSVEntry) -> Self {
        value.to_row()
    }
}

impl<T> PersistableLogger<T> for CSVLogger<T>
where
    for<'a> &'a T: Into<CSVEntry>,
{
    fn log(&mut self, entry: T) {
        self.entries.push(entry)
    }

    fn flush(&mut self) -> Result<u64> {
        let rows = self
            .entries
            .iter()
            .map(|entry| {
                let csv_entry: CSVEntry = entry.into();
                csv_entry.to_row()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let headers = self.headers.join("\t");

        let mut output_file = File::create(&self.output_filename)?;
        output_file.write_all(headers.as_bytes())?;
        output_file.write_all(b"\n")?;
        output_file.write_all(rows.as_bytes())?;

        Ok(self.entries.len() as u64)
    }
}
