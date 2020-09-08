# Pactorio

[![Version](https://img.shields.io/crates/v/pactorio?style=flat-square)][Crate]
[![Downloads](https://img.shields.io/crates/d/pactorio?style=flat-square)][Crate]
[![License](https://img.shields.io/crates/l/pactorio?style=flat-square)](https://github.com/figsoda/pactorio/blob/master/LICENSE)
[![CI](https://img.shields.io/github/workflow/status/figsoda/pactorio/ci?label=CI&logo=github&style=flat-square)](https://github.com/figsoda/pactorio/actions?query=workflow:ci)

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

## Configuration
By default, pactorio uses `pactorio.toml` as its config file. 

It is similar to the `info.json` file and uses [TOML](https://toml.io) syntax. 

Here is an example of a pactorio config file. 

```toml
# Information about your package, similar to info.json
# https://wiki.factorio.com/Tutorial:Mod_structure#info.json
[package]

# Mandatory, internal name of your mod
name = "example_mod"

# Mandatory, version of your mod, "main.major.minor"
version = "0.1.0"

# Mandatory, display name of your mod
title = "Example mod"

# Mandatory, author of your mod, You
author = "You"

# Optional, for example your email address
contact = "you@example.com"

# Optional, link to the homepage of your mod
homepage = "https://you.example.com"

# Optional, short description of your mod
description = "Example mod. Does something and some other things. "

# Optional, default to "0.12", usually needs to be added
factorio_version = "1.0"



# Dependencies are separated into four parts, All four of them are optional
# They are under a table named "deps", you can also use "dependencies"


# Required dependencies
[deps.required]

# Equal to "base >= 1.0" in info.json
base = ">= 1.0"


# Conflict or incompatible dependencies
[deps.conflict] # or [deps.incompatible]

# Equal to "! bad_mod" in info.json
bad_mod = ""

# Equal to "! incomp < 3" in info.json
incomp = "< 3"


# Optional dependencies
[deps.optional]

# Equal to "opt_dep = 0.2.1" in info.json
opt_dep = "= 0.2.1"


# Hidden optional dependencies
[deps.hidden]

# Equal to "hidden_dep > 2.0" in info.json
hidden_dep = "> 2.0"


# Source directory
# Optional, default to "src"
[source]

# Pactorio will use all the files under this directory
dir = "src"
```

</details>

## Changelog
See [CHANGELOG.md](https://github.com/figsoda/pactorio/blob/master/CHANGELOG.md)
