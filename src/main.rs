// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stdin, stdout, Read, Result, Write};
use std::path::PathBuf;

use bio::io::fasta;

use structopt::StructOpt;

/// A RecordWriter writes FASTA records to the underlying source.
///
/// # Arguments
/// * `f` - The FASTA recored to write.
pub trait RecordWriter {
    fn write_fasta_record(&mut self, f: fasta::Record) -> Result<()>;
}

/// JsonRecordWriter holds a writer, and outputs FASTA records as newline delimited json.
struct JsonRecordWriter<W: Write> {
    writer: W,
}

impl<W: Write> JsonRecordWriter<W> {
    /// Creates a new JsonRecordWriter with a writer.
    pub fn new(w: W) -> Self {
        Self { writer: w }
    }
}

impl<W: Write> RecordWriter for JsonRecordWriter<W> {
    /// Writes an input FASTA to the underlying writer.
    fn write_fasta_record(&mut self, f: fasta::Record) -> Result<()> {
        let j = serde_json::to_string(&f)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all("\n".as_bytes())?;

        Ok(())
    }
}

/// The Enum that represents the underlying CLI.
#[derive(Debug, StructOpt)]
#[structopt(name = "brrrr", about = "A biological sequence toolkit for ML.")]
enum Brrrr {
    #[structopt(name = "fa2json", about = "Converts a FASTA input to jsonl.")]
    Fa2jsonl {
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
    },
}

/// Converts a FASTA to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
fn fa2json<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    let reader = fasta::Reader::new(input);
    let writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        writer
            .write_fasta_record(record)
            .expect("Error writing record.");
    }
    Ok(())
}

fn main() {
    match Brrrr::from_args() {
        Brrrr::Fa2jsonl { input } => match input {
            None => {
                fa2json(stdin(), stdout()).expect("Error converting to jsonl.");
            }
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                fa2json(f, stdout()).expect("Error converting to jsonl.");
            }
        },
    }
}
