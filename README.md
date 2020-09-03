# Pactorio
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

### OPTIONS
    -i, --input <input>      Specify the config file to use [default: pactorio.toml]
    -o, --output <output>    Specify the output directory [default: release]
