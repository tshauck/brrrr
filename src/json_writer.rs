// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::io::{Result, Write};

use crate::writer;
use bio::io::fasta;
use bio::io::fastq;
use bio::io::gff;

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
    fn write_fasta_record(&mut self, f: fasta::Record) -> Result<()> {
        let j = serde_json::to_string(&f)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }

    /// Writes an input GFF file to the underlying writer.
    fn write_gff_record(&mut self, f: gff::Record) -> Result<()> {
        let j = serde_json::to_string(&f)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }

    /// Writes an fastq file to the underlying writer.
    fn write_fastq_record(&mut self, f: fastq::Record) -> Result<()> {
        let j = serde_json::to_string(&f)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }
}
