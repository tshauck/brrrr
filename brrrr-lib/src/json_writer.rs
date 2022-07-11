// (c) Copyright 2020 Trent Hauck
// All Rights Reserved
/// The `json_writer` module provides an implementation for the `RecordWriter` interface to read
/// and write from json.
use std::io::{BufRead, ErrorKind, Result, Write};

use serde::ser::Serialize;

use crate::types::FastaRecord;
use crate::types::FastqRecord;
use crate::types::GffRecord;
use crate::writer;

use writer::RecordWriter;

use noodles::fasta;
use noodles::fastq;
use noodles::gff;

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
        serde_json::to_writer(&mut self.writer, &r)?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }
}

/// Converts a FASTQ file to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the BufRead trait.
/// * `output` an output that implements the Write trait.
pub fn fq2jsonl<R: BufRead, W: Write>(input: R, output: &mut W) -> Result<()> {
    let mut reader = fastq::Reader::new(input);
    let record_writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = record_writer.write_serde_record(FastqRecord::from(record));

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
pub fn fa2jsonl<R: BufRead, W: Write>(input: R, output: &mut W) -> Result<()> {
    let mut reader = fasta::Reader::new(input);
    let record_writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = record_writer.write_serde_record(FastaRecord::from(record));

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
pub fn gff2jsonl<R: BufRead, W: Write>(input: R, output: &mut W) -> Result<()> {
    let mut reader = gff::Reader::new(input);
    let record_writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = record_writer.write_serde_record(GffRecord::from(record));

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(e),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fa2jsonl() {
        let input = b">A\nATCG\n" as &[u8];

        let mut output = Vec::new();
        fa2jsonl(input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        let expected_output = "{\"id\":\"A\",\"desc\":null,\"seq\":\"ATCG\"}\n".to_string();
        assert_eq!(output_str, expected_output);
    }
}
