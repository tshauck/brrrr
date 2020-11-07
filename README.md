# brrrr <a href='https://github.com/tshauck/brrrr'><img src='docs/brrrr-logo.png' align="right" height="150" /></a>

Fast command line tool to process biological sequences and annotations to modern
file formats.

- [Usage](#usage)
- [Installation](#installation)
  - [Brew](#brew)
  - [Docker](#docker)
  - [Github Releases](#github-releases)

## Usage

For a self-contained example script, see [examples.sh]. But as a quick example,
say you have a fasta file and would like to convert it to json.

```console
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
```

For the CLI help screen.

```console
$ brrrr --help
```

## Installation

There are a few different ways to install brrrr.

### Homebrew

For Macs, brew can be used.

```console
$ brew tap tshauck/brrrr
$ brew install brrrr
```

### Docker

Cross-platform builds are available from docker hub.

```console
$ docker pull thauck/brrrr
```

### Github Releases

Executables are built for:

- `x86_64-apple-darwin`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

Download the executable from github's
[release](https://github.com/tshauck/brrrr/releases) page.

[examples.sh]: https://github.com/tshauck/brrrr/blob/master/examples.sh
