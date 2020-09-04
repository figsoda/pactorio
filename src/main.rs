mod types;
use types::{Config, Info};

use anyhow::{Context, Result};
use serde::Serialize;
use structopt::StructOpt;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

/// Factorio mod packager https://github.com/figsoda/pactorio
#[derive(StructOpt)]
#[structopt(name = "pactorio")]
struct Opt {
    /// Output info.json compactly
    #[structopt(short, long)]
    compact: bool,

    /// Specify the config file to use
    #[structopt(short, long, default_value = "pactorio.toml")]
    input: String,

    /// Specify the output directory
    #[structopt(short, long, default_value = "release")]
    output: String,

    /// Output a zip file instead
    #[structopt(short, long)]
    zip: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let cfg: Config = toml::from_str(
        &fs::read_to_string(&opt.input)
            .context(format!("Failed to read the config file {}", opt.input))?,
    )
    .context(format!("Failed to parse the config file {}", opt.input))?;

    let mut files = Vec::new();
    for entry in WalkDir::new(&cfg.source.dir).min_depth(1) {
        files.push(
            entry
                .context(format!(
                    "Failed when traversing the source directory {}",
                    cfg.source.dir,
                ))?
                .path()
                .to_owned(),
        );
    }

    let info = Info::from(cfg.clone());
    let info = if opt.compact {
        serde_json::to_vec(&info).context("Failed to generate info.json")?
    } else {
        let mut writer = Vec::with_capacity(256);
        info.serialize(&mut serde_json::Serializer::with_formatter(
            &mut writer,
            serde_json::ser::PrettyFormatter::with_indent(b"    "),
        ))
        .context("Failed to generate info.json")?;
        writer
    };

    if opt.zip {
        fs::create_dir_all(&opt.output)
            .context(format!("Failed to create directory {}", &opt.output))?;
        let output = Path::new(&opt.output)
            .join(format!("{}_{}.zip", cfg.package.name, cfg.package.version));
        if output.is_dir() {
            fs::remove_dir_all(&output)
                .context(format!("Failed to remove directory {}", output.display()))?;
        } else if output.is_file() {
            fs::remove_file(&output)
                .context(format!("Failed to remove file {}", output.display()))?;
        }

        let mut zip = ZipWriter::new(
            File::create(&output)
                .context(format!("Failed to create zip file {}", output.display()))?,
        );
        let fo = FileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o755);

        for from in files {
            if let Ok(to) = from.strip_prefix(&cfg.source.dir) {
                if from.is_dir() {
                    zip.add_directory_from_path(to, Default::default())
                        .context("Failed to write to the zip file")?;
                } else if from.is_file() {
                    let mut file = File::open(&from)
                        .context(format!("Failed to read file {}", from.display()))?;
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
    } else {
        let output =
            Path::new(&opt.output).join(format!("{}_{}", cfg.package.name, cfg.package.version));
        if output.is_dir() {
            fs::remove_dir_all(&output)
                .context(format!("Failed to remove directory {}", output.display()))?;
        } else if output.is_file() {
            fs::remove_file(&output)
                .context(format!("Failed to remove file {}", output.display()))?;
        }
        fs::create_dir_all(&output)
            .context(format!("Failed to create directory {}", output.display()))?;

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
    }

    Ok(())
}
