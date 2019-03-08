# LeanPub Commandline Tool

## Introduction

This is a CLI for leanpub.com. It calls the LeanPub API to generate preview files and download them to the current directory.

## Installation

### With Homebrew (macOS and Linux)

```
brew install hjiang/tools/leanpub-cli-bin
```

### Direct download

Binaries for various platforms can be downloaded from the [releases page](https://github.com/hjiang/leanpub-cli/releases)

You can also use the following shell command to install the latest version for your platform.
```
curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git hjiang/leanpub-cli
```

### With Cargo

```
cargo install leanpub-cli
```

## Usage

The following arguments are supported:

- `--api_key`: LeanPub [API key](https://leanpub.com/author_dashboard/settings)
- `--slug`: LeanPub book slug
- `--gen_type` or `-t`: `subset` or `full`

## Contributing

Please feel free to submit issues or pull requests.
