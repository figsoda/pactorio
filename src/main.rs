mod publish;
mod release;
mod types;

use crate::types::{Config, Info};

use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::Serialize;
use structopt::StructOpt;
use walkdir::WalkDir;

use std::{
    fs::{self, File},
    io::Cursor,
    path::{Path, PathBuf},
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

    /// Publish to mod portal
    #[structopt(short, long)]
    publish: bool,

    /// Output a zip file instead
    #[structopt(short, long)]
    zip: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
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

    let file_name = format!("{}_{}", cfg.package.name, cfg.package.version);
    if opt.publish {
        let mod_name = &cfg.package.name;
        let mod_version = &cfg.package.version;

        if publish::check_mod(mod_name, mod_version.clone())
            .await
            .context(format!("Failed to query mod {}", mod_name))?
        {
            bail!("{} v{} already exists", mod_name, mod_version);
        }

        let client = Client::builder()
            .cookie_store(true)
            .build()
            .context("Failed to create http client")?;

        let csrf_token = publish::get_csrf_token(&client)
            .await
            .context("Failed to fetch csrf token")?;

        println!("Login to Factorio");
        publish::login(&client, csrf_token)
            .await
            .context("Failed to login to Factorio")?;
        println!("Processing...");

        let upload_token = publish::get_upload_token(&client, mod_name)
            .await
            .context("Failed to fetch upload token")?;

        let mut zip = Cursor::new(Vec::with_capacity(256));
        release::zip(files, info, cfg.clone(), &mut zip, file_name.into())?;
        publish::update_mod(&client, mod_name, upload_token, zip.into_inner())
            .await
            .context("Failed to update mod")?;
    } else if opt.zip {
        fs::create_dir_all(&opt.output)
            .context(format!("Failed to create directory {}", opt.output))?;
        let output = Path::new(&opt.output).join(format!("{}.zip", file_name));
        if output.is_dir() {
            fs::remove_dir_all(&output)
                .context(format!("Failed to remove directory {}", output.display()))?;
        } else if output.is_file() {
            fs::remove_file(&output)
                .context(format!("Failed to remove file {}", output.display()))?;
        }
        let file = File::create(&output)
            .context(format!("Failed to create zip file {}", output.display()))?;

        release::zip(files, info, cfg, file, PathBuf::from(file_name))?;
    } else {
        let output = Path::new(&opt.output).join(file_name);
        if output.is_dir() {
            fs::remove_dir_all(&output)
                .context(format!("Failed to remove directory {}", output.display()))?;
        } else if output.is_file() {
            fs::remove_file(&output)
                .context(format!("Failed to remove file {}", output.display()))?;
        }
        fs::create_dir_all(&output)
            .context(format!("Failed to create directory {}", output.display()))?;

        release::folder(files, info, cfg, output)?;
    }

    Ok(())
}
