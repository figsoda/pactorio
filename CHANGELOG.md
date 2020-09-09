# Changelog

---

## 0.4.2 - 2020-09-10

### Changes
- `pactorio -pz` now outputs a zip in addition to publishing to mod portal

### Documentation
- Configuration instructions in [README.md](https://github.com/figsoda/pactorio/blob/master/README.md)

---

## 0.4.1 - 2020-09-08

### Changes
- Switched from native-tls to rustls, no longer depend on openssl on linux

### Features
- Check if mod version already exist before publishing
- Check if the mod got published successfully

---

## 0.4.0 - 2020-09-06

### Changes
- Output zip files no longer have comments

### Features
- Option to publish the mod to mod portal directly

---

## 0.3.2 - 2020-09-06

### Fixes
- Now outputs the zip with the correct mod structure

---

## 0.3.1 - 2020-09-05

### Documentation
- Updated outdated README.md

### Optimization
- Revamped code for future extensions

---

## 0.3.0 - 2020-09-04

### Features
- Option to output zip files instead of folders

### Fixes
- Fixed error messages when failing to create a folder
