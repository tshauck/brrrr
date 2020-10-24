// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::io::{Result, Write};

use serde::ser::Serialize;

use crate::writer;

/// JsonRecordWriter holds a writer, and outputs FASTA records as newline delimited json.
pub struct JsonRecordWriter<W: Write> {
    writer: W,
}

impl<W: Write> JsonRecordWriter<W> {
    /// Creates a new JsonRecordWriter with a writer.
    pub fn new(w: W) -> Self {
        Self { writer: w }
    }
}

impl<W: Write> writer::RecordWriter for JsonRecordWriter<W> {
    /// Writes an input FASTA to the underlying writer.
    fn write_serde_record<S: Serialize>(&mut self, r: S) -> Result<()> {
        let j = serde_json::to_string(&r)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }
}
