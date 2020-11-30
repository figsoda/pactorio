# Pactorio

[![release](https://img.shields.io/github/v/release/figsoda/pactorio?logo=github&style=flat-square)](https://github.com/figsoda/pactorio/releases)
[![version](https://img.shields.io/crates/v/pactorio?logo=rust&style=flat-square)][Crate]
[![dependencies](https://img.shields.io/librariesio/release/cargo/pactorio?style=flat-square)](https://libraries.io/cargo/pactorio)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square)](https://www.mozilla.org/en-US/MPL/2.0)
[![ci](https://img.shields.io/github/workflow/status/figsoda/pactorio/ci?label=ci&logo=github-actions&style=flat-square)](https://github.com/figsoda/pactorio/actions?query=workflow:ci)

Pactorio is a tool that packages factorio mods and uses toml for config files.


## Installation

The latest precompiled binaries are available on [github](https://github.com/figsoda/pactorio/releases/latest).

Alternatively you can install pactorio from [crates.io][Crate] with cargo.
This requires the nightly toolchain of [Rust].

```sh
cargo +nightly install pactorio
```


## Building from source

This requires the nightly toolchain of [Rust].

```sh
cargo +nightly build --release
```


## Usage

```sh
pactorio [FLAGS] [OPTIONS]
```

flag | description
-|-
-c, --compact | Output info.json compactly
-h, --help | Prints help information
-V, --version | Prints version information
-z, --zip | Output a zip file instead

option | description
-|-
--compression \<method> | Specify the compression method, ignored without `-z/--zip` flag <br /> default: stored <br /> possible values: stored, bz2, deflate
-d, --dir \<directory> | Set working directory
-i, --input \<file> | Specify the config file to use <br /> default: pactorio.toml
-o, --output \<directory> | Specify the output directory <br /> default: release
-p, --publish \<credential>... | Publish to mod portal, accepts up to two arguments for username and password


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

# Optional, default: "0.12", usually needs to be added
factorio_version = "1.0"



# Dependencies are separated into four parts, All four of them are optional
# They are under a table named "deps", you can also use "dependencies"


# Required dependencies
[deps.required]

# Same as "base >= 1.0" in info.json
base = ">= 1.0"


# Conflict or incompatible dependencies
[deps.conflict] # or [deps.incompatible]

# Same as "! bad_mod" in info.json
bad_mod = ""

# Same as "! incomp < 3" in info.json
incomp = "< 3"


# Optional dependencies
[deps.optional]

# Same as "opt_dep = 0.2.1" in info.json
opt_dep = "= 0.2.1"


# Hidden optional dependencies
[deps.hidden]

# Same as "hidden_dep > 2.0" in info.json
hidden_dep = "> 2.0"



# Optional, source directory
[source]

# Pactorio will use all the files under this directory
# Optional, default: "src"
dir = "src"

# A list of glob patterns to represent the files to include
# Relative to your source directory
# Optional, default: ["**/**"]
include = ["**/**"]

# A list of glob patterns to represent the files to ignore
# Relative to your source directory
# Optional, default: []
ignore = []
```


## Changelog

See [CHANGELOG.md](https://github.com/figsoda/pactorio/blob/main/CHANGELOG.md)


[Crate]: https://crates.io/crates/pactorio
[Rust]: https://www.rust-lang.org/tools/install
