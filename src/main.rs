// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stdin, stdout, Result};
use std::path::PathBuf;

use bio::io::gff;
use clap::{App, Clap, IntoApp};

mod json_writer;
mod writer;

use clap_generate::generators::{Bash, Fish, PowerShell, Zsh};
use clap_generate::{generate, Generator};

#[derive(Clap, Debug, PartialEq)]
enum GeneratorChoice {
    Bash,
    Fish,
    #[clap(name = "powershell")]
    PowerShell,
    Zsh,
}

/// The Enum that represents the underlying CLI.
#[derive(Clap)]
#[clap(
    name = "brrrr",
    about = "Convert biology and chemistry domain formats into common formats.",
    author = "Trent Hauck <trent@trenthauck.com>",
    version = "0.7.3"
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
    #[clap(name = "fq2jsonl", about = "Converts a FASTQ input to jsonl.")]
    Fq2jsonl {
        #[clap(parse(from_os_str))]
        input: Option<PathBuf>,
    },
    #[clap(name = "gen", about = "Generate the man page for the tool.")]
    Completion {
        #[clap(short, long, arg_enum)]
        gen_type: GeneratorChoice,
    },
}

fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut stdout());
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
        Brrrr::Completion { gen_type } => {
            let mut app = Brrrr::into_app();
            match gen_type {
                GeneratorChoice::Bash => print_completions::<Bash>(&mut app),
                GeneratorChoice::Fish => print_completions::<Fish>(&mut app),
                GeneratorChoice::PowerShell => print_completions::<PowerShell>(&mut app),
                GeneratorChoice::Zsh => print_completions::<Zsh>(&mut app),
            }
            Ok(())
        }
    }
}
