# LeanPub Commandline Tool

## Introduction

This is a CLI for leanpub.com. It calls the LeanPub API to generate preview files and download them to the current directory.

## Installation

- Install [Rust](https://www.rust-lang.org/tools/install)
- Clone this repo
- run `cargo build --release`. The binary will be in `target/release/leanpub-cli`.

TODO: Make binary release for non-developers.

## Usage

The following arguments are supported:

- `--api_key`: LeanPub [API key](https://leanpub.com/author_dashboard/settings)
- `--slug`: LeanPub book slug
- `--gen_type` or `-t`: `subset` or `full`

## Contributing

Please feel free to submit issues or pull requests.
