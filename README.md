# brrrr <a href='https://github.com/tshauck/brrrr'><img src='brrrr/docs/brrrr-logo.png' align="right" height="150" /></a>

> brrrr is a command-line tool to facilitate common informatics tasks, and brrrr-lib is the
> abstracted code in a cargo installable package.

* [brrrr](#brrrr)
  * [Usage](#usage)
  * [Installation](#installation)
  * [Docker](#docker)
  * [GitHub Releases](#github-releases)
* [brrrr-lib](#brrrr-lib)

## brrrr

The command-line tool exposes many of the related `brrrr`  through a command line
interface. `brrrr-lib` is intended for use in other modules.

For the command-line tool help screen.

    brrrr --help

> **Warning**
> brrrr is very alpha, and may change or break at any time.

### Use Case

Use Cases:

* Convert FASTA to json
* Convert FASTA to and from parquet

#### Convert FASTA to json

As a quick example, say you have a FASTA file and would like to convert it to
json.

    $ echo ">1\nATCG\n>2\nTAGC\n" | brrrr fa2jsonl | jq
    {
      "id": "1",
      "desc": null,
      "seq": "ATCG"
    }
    {
      "id": "2",
      "desc": null,
      "seq": "TAGC"
    }

#### Convert FASTA to and from parquet

Parquet is a useful file format for large scale data storage, and there exist
many tools that can interact with it. For example, DuckDB can be used to query
parquet files with SQL.

Starting with the swissprot dataset, use the excellent seqkit to find some
summary stats.

    $ seqkit stats uniprot-reviewed_yes.fasta
    file                        format  type     num_seqs      sum_len  min_len  avg_len  max_len
    uniprot-reviewed_yes.fasta  FASTA   Protein   561,176  201,758,313        2    359.5   35,213

Convert it to parquet:

```console
$ brrrr fa2pq ./uniprot-reviewed_yes.fasta swissprot.parquet && \
    test -f swissprot.parquet && \
    echo "swissprot.parquet exists"
swissprot.parquet exists
```

Load it into DuckDB, select sequences 1000aa and over in length, then create new parquet file.

```console
$ duckdb -c "COPY (SELECT * FROM 'swissprot.parquet' WHERE length("sequence") >= 1000) TO 'swissprot.1000.parquet' (FORMAT PARQUET);"
```

```console
$ duckdb -c "SELECT COUNT(*) FROM 'swissprot.1000.parquet'"
┌──────────────┐
│ count_star() │
├──────────────┤
│ 18236        │
└──────────────┘
```

Take it from parquet and convert it back to FASTA, then confirm the `min_len` is
what's expected.

```console
$ brrrr pq2fa swissprot.1000.parquet swissprot.1000.fasta && seqkit stats swissprot.1000.fasta
file                  format  type     num_seqs     sum_len  min_len  avg_len  max_len
swissprot.1000.fasta  FASTA   Protein    18,236  28,228,604    1,000    1,548   35,213
```

### Installation

The command-line tool is the executable entrypoint, though the library can be separately
installed.

## Command-line tool

Executables are built for:

* `x86_64-apple-darwin`
* `x86_64-unknown-linux-musl`
* `aarch64-unknown-linux-gnu`
* `x86_64-pc-windows-msvc`

Download the executable from GitHub's
[release](https://github.com/tshauck/brrrr/releases/latest) page.

[examples.sh]: https://github.com/tshauck/brrrr/blob/main/examples.sh

## brrrr-lib

`brrrr-lib` is a crate contains abstracted code and is used by the command-line tool.

```toml
[dependencies]
brrrr-lib = "0.14.0"
```

Docs are available here: [docs.rs/brrrr-lib](https://docs.rs/brrrr-lib/0.9.11/brrrr_lib/).
