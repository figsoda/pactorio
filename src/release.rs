use anyhow::{anyhow, Context, Result};
use is_executable::IsExecutable;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::fail;

use std::{
    fs::{self, File},
    io::{self, Seek, Write},
    path::{Path, PathBuf},
};

pub fn remove_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path).with_context(fail::remove_dir(path.display()))?;
    } else if path.is_file() {
        fs::remove_file(path).with_context(fail::remove_file(path.display()))?;
    }

    Ok(())
}

pub fn folder(files: Vec<(PathBuf, PathBuf)>, info: Vec<u8>, output: &Path) -> Result<()> {
    for (from, to) in files {
        let to = output.join(to);
        let todir = to
            .parent()
            .ok_or_else(|| anyhow!("Cannot find the parent of {}", to.display()))?;

        if from.is_file() {
            fs::create_dir_all(todir).with_context(fail::create_dir(todir.display()))?;
            fs::copy(&from, &to).with_context(fail::copy_file(from.display(), to.display()))?;
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
    compression: CompressionMethod,
) -> Result<()> {
    let mut zip = ZipWriter::new(writer);
    zip.set_comment("");

    let regular = FileOptions::default().compression_method(compression);
    let executable = regular.unix_permissions(0o755);

    for (from, to) in files {
        let to = root.join(to);
        if from.is_file() {
            let mut file = File::open(&from).with_context(fail::read(from.display()))?;
            zip.start_file(
                to.to_string_lossy(),
                if from.is_executable() {
                    executable
                } else {
                    regular
                },
            )
            .context("Failed to write to the zip file")?;
            io::copy(&mut file, &mut zip).context("Failed to write to the zip file")?;
        }
    }

    zip.start_file(root.join("info.json").to_string_lossy(), regular)
        .context("Failed to write to the zip file")?;
    zip.write_all(&info)
        .context("Failed to write to the zip file")?;
    zip.finish().context("Failed to write to the zip file")?;

    Ok(())
}
