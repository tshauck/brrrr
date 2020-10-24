// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::io::{ErrorKind, Read, Result, Write};

use serde::ser::Serialize;

use crate::writer;

use writer::RecordWriter;

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
    fn write_serde_record<S: Serialize>(&mut self, r: S) -> Result<()> {
        let j = serde_json::to_string(&r)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }
}

/// Converts a FASTA file to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
pub fn fq2jsonl<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    let reader = fastq::Reader::new(input);
    let record_writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = record_writer.write_serde_record(record);

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(e),
            }
        }
    }
    Ok(())
}

/// Converts a FASTA to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
pub fn fa2jsonl<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    let reader = fasta::Reader::new(input);
    let record_writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = record_writer.write_serde_record(record);

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(e),
            }
        }
    }
    Ok(())
}

/// Converts a GFF file to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
/// * `gff_type` the underlying gff type.
pub fn gff2jsonl<R: Read, W: Write>(input: R, output: W, gff_type: gff::GffType) -> Result<()> {
    let mut reader = gff::Reader::new(input, gff_type);
    let record_writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = record_writer.write_serde_record(record);

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(e),
            }
        }
    }
    Ok(())
}
