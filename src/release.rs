use anyhow::{Context, Result};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use std::{
    fs::{self, File},
    io::{self, Seek, Write},
    path::PathBuf,
};

pub fn folder(files: Vec<(PathBuf, PathBuf)>, info: Vec<u8>, output: PathBuf) -> Result<()> {
    for (from, to) in files {
        let to = output.join(to);
        if from.is_dir() {
            fs::create_dir(&to).context(format!("Failed to create directory {}", to.display()))?;
        } else if from.is_file() {
            fs::copy(&from, &to).context(format!(
                "Failed to copy from {} to {}",
                from.display(),
                to.display(),
            ))?;
        }
    }

    fs::write(output.join("info.json"), info).context("Failed to create file info.json")?;

    Ok(())
}

pub fn zip(
    files: Vec<(PathBuf, PathBuf)>,
    info: Vec<u8>,
    writer: impl Write + Seek,
    root: PathBuf,
) -> Result<()> {
    let mut zip = ZipWriter::new(writer);
    zip.set_comment("");

    let fo = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    for (from, to) in files {
        let to = root.join(to);
        if from.is_dir() {
            zip.add_directory(to.to_string_lossy(), Default::default())
                .context("Failed to write to the zip file")?;
        } else if from.is_file() {
            let mut file =
                File::open(&from).context(format!("Failed to read file {}", from.display()))?;
            zip.start_file(to.to_string_lossy(), fo)
                .context("Failed to write to the zip file")?;
            io::copy(&mut file, &mut zip).context("Failed to write to the zip file")?;
        }
    }

    zip.start_file(root.join("info.json").to_string_lossy(), fo)
        .context("Failed to write to the zip file")?;
    zip.write_all(&info)
        .context("Failed to write to the zip file")?;
    zip.finish().context("Failed to write to the zip file")?;

    Ok(())
}
