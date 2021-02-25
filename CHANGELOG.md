# Changelog

## v0.5.1 - 2021-02-25

### Features
- Completions for bash, elvish, fish, powershell, and zsh


## v0.5.0 - 2020-11-19

### Changes
- Print information to stderr instead of stdout

### Optimization
- Reduce size of binary


## v0.4.8 - 2020-11-05

### Features
- Option to specify the compression method


## v0.4.7 - 2020-10-27

### Fixes
- Fixed incompatibility due to tokio crate


## v0.4.6 - 2020-10-18

### Features
- Option to set working directory with `-d` or `--dir`


## v0.4.5 - 2020-09-20

### Features
- `pactorio -p` can now accept up to two arguments for factorio login credentials

### Fixes
- Now outputs correct error messages when the version of the mod already exist


## v0.4.4 - 2020-09-13

### Features
- New config `source.ignore` to ignore files with glob patterns
- Help information is now colored


## v0.4.3 - 2020-09-10

### Features
- New config `source.include` to filter source directory with glob patterns


## v0.4.2 - 2020-09-09

### Changes
- `pactorio -pz` now outputs a zip in addition to publishing to mod portal

### Documentation
- Configuration instructions in [README.md](README.md)


## v0.4.1 - 2020-09-08

### Changes
- Switched from native-tls to rustls, no longer depend on openssl on linux

### Features
- Check if mod version already exist before publishing
- Check if the mod got published successfully


## v0.4.0 - 2020-09-06

### Changes
- Output zip files no longer have comments

### Features
- Option to publish the mod to mod portal directly


## v0.3.2 - 2020-09-06

### Fixes
- Now outputs the zip with the correct mod structure


## v0.3.1 - 2020-09-05

### Documentation
- Updated outdated README.md

### Optimization
- Revamped code for future extensions


## v0.3.0 - 2020-09-04

### Features
- Option to output zip files instead of folders

### Fixes
- Fixed error messages when failing to create a folder
