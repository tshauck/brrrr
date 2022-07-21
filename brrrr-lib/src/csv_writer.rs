// (c) Copyright 2020 Trent Hauck
// All Rights Reserved
/// The `csv_writer` module provides an implementation for the `RecordWriter` interface to read
/// and write from csvs.
use std::io::{self, BufRead, ErrorKind, Write};

use serde::ser::Serialize;

use crate::errors::BrrrrError;
use crate::types::{FastaRecord, FastqRecord};
use crate::writer;

use writer::RecordWriter;

use noodles::fasta;
use noodles::fastq;

/// CsvRecordWriter holds a writer, and outputs FASTA records as newline delimited json.
pub struct CsvRecordWriter<W: Write> {
    csv_writer: csv::Writer<W>,
}

impl<W: Write> CsvRecordWriter<W> {
    /// Creates a new CsvRecordWriter with a writer.
    pub fn new(w: W) -> Self {
        let csv_writer = csv::Writer::from_writer(w);
        Self { csv_writer }
    }
}

impl<W: Write> writer::RecordWriter for CsvRecordWriter<W> {
    /// Writes an input serializable object to the underlying writer.
    fn write_serde_record<S: Serialize>(&mut self, r: S) -> io::Result<()> {
        self.csv_writer.serialize(r)?;
        Ok(())
    }
}

/// Converts a FASTA to CSV
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
pub fn fa2csv<R: BufRead, W: Write>(input: R, output: &mut W) -> Result<(), BrrrrError> {
    let mut reader = fasta::Reader::new(input);
    let record_writer = &mut CsvRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record?;
        let write_op = record_writer.write_serde_record(FastaRecord::from(record));

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(BrrrrError::IOError(e)),
            }
        }
    }
    Ok(())
}

/// Converts a FASTQ file to CSV
///
/// # Arguments
///
/// * `input` an input that implements the BufRead trait.
/// * `output` an output that implements the Write trait.
pub fn fq2csv<R: BufRead, W: Write>(input: R, output: &mut W) -> Result<(), BrrrrError> {
    let mut reader = fastq::Reader::new(input);
    let record_writer = &mut CsvRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record?;
        let write_op = record_writer.write_serde_record(FastqRecord::from(record));

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(BrrrrError::IOError(e)),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fa2csv() {
        let input = b">A\nATCG\n" as &[u8];

        let mut output = Vec::new();
        fa2csv(input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        let expected_output = "id,desc,seq\nA,,ATCG\n".to_string();
        assert_eq!(output_str, expected_output);
    }
}
