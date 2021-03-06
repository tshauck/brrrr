# brrrr <a href='https://github.com/tshauck/brrrr'><img src='brrrr/docs/brrrr-logo.png' align="right" height="150" /></a>

Fast command line tool to process biological sequences and annotations to modern
file formats.

- [brrrr](#brrrr)
  - [Usage](#usage)
  - [Installation](#installation)
  - [Docker](#docker)
  - [GitHub Releases](#github-releases)
- [brrrr-lib](#brrrr-lib)

## brrrr

The CLI exposes many of the related `brrrr` functionality through a command line interface. `brrrr-lib` is intended for use in other modules.

### Usage

As a quick example, say you have a FASTA file and would like to convert it to
json.

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

### Installation

The `brrrr` cli can be install one of two ways, either through Docker or by
getting an executable via GitHub's release page.

### Docker

Cross-platform builds are available from docker hub.

```console
$ docker pull thauck/brrrr
```

### GitHub Releases

Executables are built for:

- `x86_64-apple-darwin`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

Download the executable from GitHub's
[release](https://github.com/tshauck/brrrr/releases) page.

[examples.sh]: https://github.com/tshauck/brrrr/blob/master/examples.sh

## brrrr-lib

`brrrr-lib` can be added to `Cargo.toml` to import the module.

```toml
[dependencies]
brrrr-lib = { git = "https://github.com/tshauck/brrrr", tag = "v1.2.3" }
```
