// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tshauck/brrrr/main/brrrr/docs/brrrr-logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/tshauck/brrrr/main/brrrr/docs/brrrr-logo.svg"
)]

//! # brrrr
//!
//! `brrrr` and in particular, `brrrr_lib`, is a library for supporting writing genomics file
//! formats in file formats that are usable by general-purpose analytics infrastructure, e.g.
//! Spark.
//!
//! ## Quick Start
//!
//! For example, to write a FASTA file to the stdout.
//!
//! ```rust
//! use std::io::stdout;
//!
//! use brrrr_lib::json_writer::fa2jsonl;
//!
//! fn main() {
//!     let example_input = b">A\nATCG\n>B\nGCTA" as &[u8];
//!     fa2jsonl(example_input, &mut stdout()).expect("Error... :(");
//! }
//! ```
//!
//! `fa2jsonl` relies on `JsonRecordWriter`, which knows how to parse the input FASTA bytes and
//! write them to objects that implement `Write`.
//!
//! If you're interested in the command-lind tool, see: <https://github.com/tshauck/brrrr/releases/latest>

/// json_writer holds a writer, and outputs FASTA and GFF records as newline delimited JSON.
pub mod json_writer;

/// csv_writer holds a writer, and outputs FASTA and GFF records as csv.
pub mod csv_writer;

/// parquet_writer holds a writer, and outputs FASTA and GFF records as parquet.
pub mod parquet_writer;

/// parquet_reader is like parquet_writer, but for reading parquet in.
pub mod parquet_reader;

/// Interface for the generic writer object.
pub mod writer;

/// Types used within the library.
pub mod types;

/// Custom brrrr errors.
pub mod errors;
