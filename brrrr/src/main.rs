// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{self, stdin, stdout, BufReader};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use brrrr_lib::csv_writer;
use brrrr_lib::json_writer;
use brrrr_lib::parquet_reader;
use brrrr_lib::parquet_writer;
use parquet::basic::Compression;

/// The Enum that represents the underlying CLI.
#[derive(Parser)]
#[clap(
    name = "brrrr",
    about = "Commandline utilities for modern biology and chemistry informatics.",
    author = "Trent Hauck <trent@trenthauck.com>",
    version = "0.12.0"
)]
struct Cli {
    #[clap(subcommand)]
    command: Brrrr,
}

#[derive(clap::ValueEnum, Clone)]
enum ParquetCompression {
    UNCOMPRESSED,
    SNAPPY,
    GZIP,
    BROTLI,
    LZO,
    LZ4,
    ZSTD,
}

impl Into<Compression> for ParquetCompression {
    fn into(self) -> Compression {
        match self {
            ParquetCompression::UNCOMPRESSED => Compression::UNCOMPRESSED,
            ParquetCompression::GZIP => Compression::GZIP,
            ParquetCompression::BROTLI => Compression::BROTLI,
            ParquetCompression::SNAPPY => Compression::SNAPPY,
            ParquetCompression::LZO => Compression::LZO,
            ParquetCompression::LZ4 => Compression::LZ4,
            ParquetCompression::ZSTD => Compression::ZSTD,
        }
    }
}

#[derive(clap::ValueEnum, Clone)]
enum CliBioFileCompression {
    UNCOMPRESSED,
    GZIP,
}

impl Into<parquet_writer::BioFileCompression> for CliBioFileCompression {
    fn into(self) -> parquet_writer::BioFileCompression {
        match self {
            CliBioFileCompression::UNCOMPRESSED => parquet_writer::BioFileCompression::UNCOMPRESSED,
            CliBioFileCompression::GZIP => parquet_writer::BioFileCompression::GZIP,
        }
    }
}

fn file_exists(p: &str) -> Result<(), String> {
    if !PathBuf::from(p).exists() {
        return Err(format!("File path {:?} does not exist", p));
    } else {
        return Ok(());
    }
}
#[derive(Subcommand)]
enum Brrrr {
    #[clap(name = "fa2pq", about = "Converts a FASTA input to parquet.")]
    Fa2pq {
        /// The path where the input should be read from.
        #[clap(validator = file_exists)]
        input_file_name: PathBuf,
        /// The path where the output should be written to.
        output_file_name: PathBuf,
        /// The compression mode for the parquet.
        #[clap(short, long, value_enum, default_value = "uncompressed")]
        output_compression: ParquetCompression,
        /// The bio file compression.
        #[clap(short, long, value_enum, default_value = "uncompressed")]
        input_compression: CliBioFileCompression,
    },
    #[clap(name = "pq2fa", about = "Converts a parquet file to FASTA format.")]
    Pq2Fa {
        /// The path where the input should be read from.
        #[clap(validator = file_exists)]
        input_file_name: PathBuf,
        /// The path where the output should be written to.
        output_file_name: PathBuf,
    },
    #[clap(name = "pq2fq", about = "Converts a parquet file to FASTQ format.")]
    Pq2Fq {
        /// The path where the input should be read from.
        #[clap(validator = file_exists)]
        input_file_name: PathBuf,
        /// The path where the output should be written to.
        output_file_name: PathBuf,
    },
    #[clap(name = "fq2pq", about = "Converts a FASTQ input to parquet.")]
    Fq2pq {
        /// The path where the input should be read from.
        #[clap(validator = file_exists)]
        input_file_name: PathBuf,
        /// The path where the output should be written to.
        output_file_name: PathBuf,
        /// The compression mode for the parquet.
        #[clap(value_enum, default_value = "uncompressed")]
        compression: ParquetCompression,
    },
    #[clap(name = "fa2jsonl", about = "Converts a FASTA input to jsonl.")]
    Fa2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[clap(name = "gff2pq", about = "Converts a GFF-like input to parquet.")]
    Gff2pq {
        /// The path where the input should be read from.
        #[clap(validator = file_exists)]
        input_file_name: PathBuf,
        /// The path where the output should be written to.
        output_file_name: PathBuf,
        /// The compression mode for the parquet.
        #[clap(value_enum, default_value = "uncompressed")]
        compression: ParquetCompression,
    },
    #[clap(name = "gff2jsonl", about = "Converts a GFF-like input to jsonl.")]
    Gff2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[clap(name = "fq2jsonl", about = "Converts a FASTQ input to jsonl.")]
    Fq2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[clap(name = "fa2csv", about = "Converts a FASTA input to csv.")]
    Fa2csv {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[clap(name = "fq2csv", about = "Converts a FASTQ input to csv.")]
    Fq2csv {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Brrrr::Fa2pq {
            input_file_name,
            output_file_name,
            output_compression,
            input_compression,
        } => parquet_writer::fa2pq(
            &input_file_name,
            &output_file_name,
            output_compression.into(),
            input_compression.into(),
        ),
        Brrrr::Pq2Fa {
            input_file_name,
            output_file_name,
        } => parquet_reader::pq2fa(input_file_name, output_file_name),
        Brrrr::Pq2Fq {
            input_file_name,
            output_file_name,
        } => parquet_reader::pq2fq(input_file_name, output_file_name),
        Brrrr::Fq2pq {
            input_file_name,
            output_file_name,
            compression,
        } => parquet_writer::fq2pq(input_file_name, output_file_name, compression.into()),
        Brrrr::Fa2csv { input } => match input {
            None => csv_writer::fa2csv(stdin().lock(), &mut stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                csv_writer::fa2csv(BufReader::new(f), &mut stdout())
            }
        },
        Brrrr::Fq2csv { input } => match input {
            None => csv_writer::fq2csv(stdin().lock(), &mut stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                csv_writer::fq2csv(BufReader::new(f), &mut stdout())
            }
        },
        Brrrr::Fa2jsonl { input } => match input {
            None => json_writer::fa2jsonl(stdin().lock(), &mut stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                json_writer::fa2jsonl(BufReader::new(f), &mut stdout())
            }
        },
        Brrrr::Gff2jsonl { input } => match input {
            None => json_writer::gff2jsonl(stdin().lock(), &mut stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                json_writer::gff2jsonl(BufReader::new(f), &mut stdout())
            }
        },
        Brrrr::Gff2pq {
            input_file_name,
            output_file_name,
            compression,
        } => parquet_writer::gff2pq(input_file_name, output_file_name, compression.into()),
        Brrrr::Fq2jsonl { input } => match input {
            None => json_writer::fq2jsonl(stdin().lock(), &mut stdout()),
            Some(input) => {
                let f = File::open(input).expect("Error opening file.");
                json_writer::fq2jsonl(BufReader::new(f), &mut stdout())
            }
        },
    }
}
