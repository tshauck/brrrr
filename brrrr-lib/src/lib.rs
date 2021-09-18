// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tshauck/brrrr/master/brrrr/docs/brrrr-logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/tshauck/brrrr/master/brrrr/docs/brrrr-logo.svg"
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
//! `fa2jsonl` relies on `JsonRecordWriter`, which knows how to parse the input fasta bytes and
//! write them to objects that implement `Write`.

/// json_writer holds a writer, and outputs FASTA and GFF records as newline delimited json.
pub mod json_writer;

/// parquet_writer holds a writer, and outputs FASTA and GFF records as parquet.
pub mod parquet_writer;

/// Interface for the generic writer object.
pub mod writer;
