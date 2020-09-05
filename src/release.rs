use crate::types::Config;

use anyhow::{Context, Result};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use std::{
    fs::{self, File},
    io::{self, Seek, Write},
    path::PathBuf,
};

pub fn folder(files: Vec<PathBuf>, info: Vec<u8>, cfg: Config, output: PathBuf) -> Result<()> {
    for from in files {
        if let Ok(to) = from.strip_prefix(&cfg.source.dir) {
            let to = output.join(to);
            if from.is_dir() {
                fs::create_dir(&to)
                    .context(format!("Failed to create directory {}", to.display()))?;
            } else if from.is_file() {
                fs::copy(&from, &to).context(format!(
                    "Failed to copy from {} to {}",
                    from.display(),
                    to.display(),
                ))?;
            }
        }
    }

    fs::write(output.join("info.json"), info).context("Failed to create file info.json")?;

    Ok(())
}

pub fn zip(
    files: Vec<PathBuf>,
    info: Vec<u8>,
    cfg: Config,
    writer: impl Write + Seek,
) -> Result<()> {
    let mut zip = ZipWriter::new(writer);
    let fo = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    for from in files {
        if let Ok(to) = from.strip_prefix(&cfg.source.dir) {
            if from.is_dir() {
                zip.add_directory_from_path(to, Default::default())
                    .context("Failed to write to the zip file")?;
            } else if from.is_file() {
                let mut file =
                    File::open(&from).context(format!("Failed to read file {}", from.display()))?;
                zip.start_file_from_path(to, fo)
                    .context("Failed to write to the zip file")?;
                io::copy(&mut file, &mut zip).context("Failed to write to the zip file")?;
            }
        }
    }

    zip.start_file("info.json", fo)
        .context("Failed to write to the zip file")?;
    zip.write_all(&info)
        .context("Failed to write to the zip file")?;
    zip.finish().context("Failed to write to the zip file")?;

    Ok(())
}
