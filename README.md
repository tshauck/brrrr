# brrrr <a href='https://github.com/tshauck/brrrr'><img src='docs/brrrr-logo.png' align="right" height="150" /></a>

Fast command line tool to process biological sequences and annotations to modern
file formats.

## Usage

```console
$ brrrr
brrrr 0.4.2
Trent Hauck <trent@trenthauck.com>
Convert domain specific files into common formats.

USAGE:
    brrrr <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    fa2jsonl     Converts a FASTA input to jsonl.
    fq2jsonl     Converts a FASTQ input to jsonl
    gff2jsonl    Converts a GFF-like input to jsonl.
    help         Prints this message or the help of the given subcommand(s)
```

## Installation

Executables are built for:

- `x86_64-apple-darwin`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-gnu`

Download the executable from github's
[release](https://github.com/tshauck/brrrr/releases) page.
