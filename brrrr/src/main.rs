// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stdin, stdout, Result};
use std::path::PathBuf;

use argh::FromArgs;

use bio::io::gff;

use brrrr_lib::csv_writer;
use brrrr_lib::json_writer;
use brrrr_lib::parquet_writer;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "fa2jsonl",
    description = "Converts a FASTA input to jsonl."
)]
struct Fa2jsonl {
    #[argh(
        positional,
        description = "the input FASTA file, if omitted stdin is used"
    )]
    input: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "fa2pq",
    description = "Converts a FASTA input to parquet."
)]
struct Fa2pq {
    #[argh(positional, description = "the input FASTA file")]
    input: String,

    #[argh(positional, description = "the output pq file where data is written")]
    output: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "fq2pq",
    description = "Converts a FASTQ input to parquet."
)]
struct Fq2pq {
    #[argh(positional, description = "the input FASTQ file")]
    input: String,

    #[argh(positional, description = "the output pq file where data is written")]
    output: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "gff2jsonl",
    description = "Converts a GFF input to jsonl."
)]
struct Gff2jsonl {
    #[argh(positional, description = "the input gff file")]
    input: Option<PathBuf>,

    #[argh(
        option,
        description = "the gff dialect",
        default = "gff::GffType::GFF3"
    )]
    gff_type: gff::GffType,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "fq2jsonl",
    description = "Converts a FASTQ input to jsonl."
)]
struct Fq2jsonl {
    #[argh(positional, description = "the input fastq file")]
    input: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "fa2csv",
    description = "Converts a GFF input to jsonl."
)]
struct Fa2csv {
    #[argh(positional, description = "the input fastq file")]
    input: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "fq2csv",
    description = "Converts a FASTQ input to csv."
)]
struct Fq2csv {
    #[argh(positional, description = "the input fastq file")]
    input: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    /// fa2jsonl
    Fa2jsonl(Fa2jsonl),
    Fa2pq(Fa2pq),
    Fq2pq(Fq2pq),
    Gff2jsonl(Gff2jsonl),
    Fq2jsonl(Fq2jsonl),
    Fa2csv(Fa2csv),
    Fq2csv(Fq2csv),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    name = "brrrr",
    description = "Bioinformatic files go brrrr. Version 0.11.2"
)]
struct Brrrr {
    #[argh(subcommand, description = "the sub-command")]
    subcommand: Subcommand,
}

fn main() -> Result<()> {
    let cmd: Brrrr = argh::from_env();

    match cmd.subcommand {
        Subcommand::Fa2jsonl(fasta2jsonl) => match fasta2jsonl.input {
            Some(path_buf) => {
                let f = File::open(path_buf)?;
                json_writer::fa2jsonl(f, &mut stdout())
            }
            None => json_writer::fa2jsonl(stdin(), &mut stdout()),
        },
        Subcommand::Fa2pq(fa2pq) => {
            parquet_writer::fa2pq(fa2pq.input.as_str(), fa2pq.output.as_str())
        }
        Subcommand::Fq2pq(fq2pq) => {
            parquet_writer::fq2pq(fq2pq.input.as_str(), fq2pq.output.as_str())
        }
        Subcommand::Gff2jsonl(gff2jsonl) => match gff2jsonl.input {
            Some(path_buf) => {
                let f = File::open(path_buf)?;
                json_writer::gff2jsonl(f, &mut stdout(), gff2jsonl.gff_type)
            }
            None => json_writer::gff2jsonl(stdin(), &mut stdout(), gff2jsonl.gff_type),
        },
        Subcommand::Fq2jsonl(fq2jsonl) => match fq2jsonl.input {
            None => json_writer::fq2jsonl(stdin(), &mut stdout()),
            Some(input) => {
                let f = File::open(input)?;
                json_writer::fq2jsonl(f, &mut stdout())
            }
        },
        Subcommand::Fa2csv(fa2csv) => match fa2csv.input {
            None => csv_writer::fa2csv(stdin(), &mut stdout()),
            Some(input) => {
                let f = File::open(input)?;
                csv_writer::fa2csv(f, &mut stdout())
            }
        },
        Subcommand::Fq2csv(fq2csv) => match fq2csv.input {
            None => csv_writer::fq2csv(stdin(), &mut stdout()),
            Some(input) => {
                let f = File::open(input)?;
                csv_writer::fq2csv(f, &mut stdout())
            }
        },
    }
}
