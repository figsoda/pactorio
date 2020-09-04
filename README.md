# Pactorio

[![version](https://img.shields.io/crates/v/pactorio.svg?style=flat-square)][Crate]
[![downloads](https://img.shields.io/crates/d/pactorio.svg?style=flat-square)][Crate]
[![license](https://img.shields.io/crates/l/pactorio.svg?style=flat-square)](https://github.com/figsoda/pactorio/blob/master/LICENSE)
[![top language](https://img.shields.io/github/languages/top/figsoda/pactorio.svg?style=flat-square)](https://www.rust-lang.org/)

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
    -V, --version    Prints version information
    -z, --zip        Output a zip file instead

### OPTIONS
    -i, --input <input>      Specify the config file to use [default: pactorio.toml]
    -o, --output <output>    Specify the output directory [default: release]

## Changelog
See [CHANGELOG.md](https://github.com/figsoda/pactorio/blob/master/CHANGELOG.md)
