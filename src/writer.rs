// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use serde::ser::Serialize;

use std::io::Result;

/// A RecordWriter writes FASTA records to the underlying source.
pub trait RecordWriter {
    fn write_serde_record<S: Serialize>(&mut self, r: S) -> Result<()>;
}
