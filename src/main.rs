// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stdin, stdout, ErrorKind, Read, Result, Write};
use std::path::PathBuf;

use bio::io::fasta;
use bio::io::fastq;
use bio::io::gff;

use structopt::StructOpt;

mod json_writer;
pub mod writer;

use writer::RecordWriter;

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
    #[structopt(name = "fq2jsonl", about = "Converts a FASTQ input to jsonl")]
    Fq2jsonl {
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
    },
}

/// Converts a FASTA file to JSONL
///
/// # Arguments
///
/// * `input` an input that implements the Read trait.
/// * `output` an output that implements the Write trait.
fn fq2jsonl<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    let reader = fastq::Reader::new(input);
    let writer = &mut json_writer::JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = writer.write_fastq_record(record);

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
fn fa2jsonl<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    let reader = fasta::Reader::new(input);
    let writer = &mut json_writer::JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = writer.write_fasta_record(record);

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
fn gff2jsonl<R: Read, W: Write>(input: R, output: W, gff_type: gff::GffType) -> Result<()> {
    let mut reader = gff::Reader::new(input, gff_type);
    let writer = &mut json_writer::JsonRecordWriter::new(output);

    for read_record in reader.records() {
        let record = read_record.expect("Error parsing record.");
        let write_op = writer.write_gff_record(record);

        if let Err(e) = write_op {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(e),
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    match Brrrr::from_args() {
        Brrrr::Fa2jsonl { input } => match input {
            None => fa2jsonl(stdin(), stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                fa2jsonl(f, stdout())
            }
        },
        Brrrr::Gff2jsonl { input, gff_type } => match input {
            None => gff2jsonl(stdin(), stdout(), gff_type),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                gff2jsonl(f, stdout(), gff_type)
            }
        },
        Brrrr::Fq2jsonl { input } => match input {
            None => fq2jsonl(stdin(), stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                fq2jsonl(f, stdout())
            }
        },
    }
}
