// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stdin, stdout, Result};
use std::path::PathBuf;

use bio::io::gff;
use clap::Clap;

mod json_writer;
mod writer;

/// The Enum that represents the underlying CLI.
#[derive(Clap)]
#[clap(
    name = "brrrr",
    about = "Convert biology and chemistry domain formats into common formats.",
    author = "Trent Hauck <trent@trenthauck.com>",
    version = "0.4.2"
)]
enum Brrrr {
    #[clap(name = "fa2jsonl", about = "Converts a FASTA input to jsonl.")]
    Fa2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[clap(name = "gff2jsonl", about = "Converts a GFF-like input to jsonl.")]
    Gff2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,

        #[clap(short, long, default_value = "gff3")]
        /// The specific GFF format: gff3, gff2, or gft
        gff_type: gff::GffType,
    },
    #[clap(name = "fq2jsonl", about = "Converts a FASTQ input to jsonl")]
    Fq2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    match Brrrr::parse() {
        Brrrr::Fa2jsonl { input } => match input {
            None => json_writer::fa2jsonl(stdin(), stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                json_writer::fa2jsonl(f, stdout())
            }
        },
        Brrrr::Gff2jsonl { input, gff_type } => match input {
            None => json_writer::gff2jsonl(stdin(), stdout(), gff_type),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                json_writer::gff2jsonl(f, stdout(), gff_type)
            }
        },
        Brrrr::Fq2jsonl { input } => match input {
            None => json_writer::fq2jsonl(stdin(), stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                json_writer::fq2jsonl(f, stdout())
            }
        },
    }
}
