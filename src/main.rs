// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stderr, stdin, stdout, ErrorKind, Read, Result, Write};
use std::path::PathBuf;
use std::process;

use bio::io::fasta;
use bio::io::gff;

use structopt::StructOpt;

/// A RecordWriter writes FASTA records to the underlying source.
pub trait RecordWriter {
    fn write_fasta_record(&mut self, f: fasta::Record) -> Result<()>;
    fn write_gff_record(&mut self, f: gff::Record) -> Result<()>;
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

    /// Writes and input GFF file to the underlying writer.
    fn write_gff_record(&mut self, f: gff::Record) -> Result<()> {
        let j = serde_json::to_string(&f)?;

        self.writer.write_all(j.as_bytes())?;
        self.writer.write_all("\n".as_bytes())?;

        Ok(())
    }
}

/// The Enum that represents the underlying CLI.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "brrrr",
    about = "A fast command line tool to process biological sequences and annotations to modern file formats."
)]
enum Brrrr {
    #[structopt(name = "fa2jsonl", about = "Converts a FASTA input to jsonl.")]
    Fa2jsonl {
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[structopt(name = "gff2jsonl", about = "Converts a GFF-like input to jsonl.")]
    Gff2jsonl {
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,

        #[structopt(short = "g", long = "gfftype", default_value = "gff3")]
        /// The specific GFF format: gff3, gff2, or gft
        gff_type: gff::GffType,
    },
}

/// Converts a FASTA to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
fn fa2jsonl<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    let reader = fasta::Reader::new(input);
    let writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = writer.write_fasta_record(record);

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => {
                    writeln!(stderr(), "{}", e).unwrap();
                    process::exit(1);
                }
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
fn gff2jsonl<R: Read, W: Write>(input: R, output: W, gff_type: gff::GffType) -> Result<()> {
    let mut reader = gff::Reader::new(input, gff_type);
    let writer = &mut JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = writer.write_gff_record(record);

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => {
                    writeln!(stderr(), "{}", e).unwrap();
                    process::exit(1);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    match Brrrr::from_args() {
        Brrrr::Fa2jsonl { input } => match input {
            None => {
                fa2jsonl(stdin(), stdout()).expect("Error converting to jsonl.");
            }
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                fa2jsonl(f, stdout()).expect("Error converting to jsonl.");
            }
        },
        Brrrr::Gff2jsonl { input, gff_type } => match input {
            None => {
                gff2jsonl(stdin(), stdout(), gff_type).expect("Error converting to jsonl.");
            }
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                gff2jsonl(f, stdout(), gff_type).expect("Error converting to jsonl.");
            }
        },
    }
}
