# Pactorio

[![Version](https://img.shields.io/crates/v/pactorio?style=flat-square)][Crate]
[![Downloads](https://img.shields.io/crates/d/pactorio?style=flat-square)][Crate]
[![License](https://img.shields.io/crates/l/pactorio?style=flat-square)](https://github.com/figsoda/pactorio/blob/master/LICENSE)
[![Status](https://img.shields.io/github/workflow/status/figsoda/pactorio/CI?style=flat-square)](https://github.com/figsoda/pactorio/actions)

[Crate]: https://crates.io/crates/pactorio

Pactorio is a tool that packages factorio mods and uses toml for config files. 

## Installing with cargo
```
cargo install pactorio
```

## Building from source
```
cargo build --release
```

## Usage
    pactorio [FLAGS] [OPTIONS]

### FLAGS
    -c, --compact    Output info.json compactly
    -h, --help       Prints help information
    -p, --publish    Publish to mod portal
    -V, --version    Prints version information
    -z, --zip        Output a zip file instead

### OPTIONS
    -i, --input <input>      Specify the config file to use [default: pactorio.toml]
    -o, --output <output>    Specify the output directory [default: release]

## Changelog
See [CHANGELOG.md](https://github.com/figsoda/pactorio/blob/master/CHANGELOG.md)
