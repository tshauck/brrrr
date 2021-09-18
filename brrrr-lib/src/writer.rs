// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use serde::ser::Serialize;

use std::io::Result;

/// A RecordWriter writes FASTA records to the underlying source.
///
/// Implement this trait in order to read bioinformatic formats and write it the paricular
/// underlying format.
///
/// # Examples
///
/// ```rust
/// // Given our `JsonRecordWriter`, implementing the RecordWriter means it's possible to
/// // write records in json to underlying structs that implement Write.
/// impl<W: Write> writer::RecordWriter for JsonRecordWriter<W> {
///     fn write_serde_record<S: Serialize>(&mut self, r: S) -> Result<()> {
///         serde_json::to_writer(&mut self.writer, &r)?;
///         self.writer.write_all(b"\n")?;
///
///         Ok(())
///     }
/// }
/// ```
pub trait RecordWriter {
    fn write_serde_record<S: Serialize>(&mut self, r: S) -> Result<()>;
}
