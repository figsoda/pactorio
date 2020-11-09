#![forbid(unsafe_code)]

mod fail;
mod publish;
mod release;
mod types;

use crate::types::{Config, Info};

use anyhow::{bail, Context, Result};
use globset::{Glob, GlobSetBuilder};
use reqwest::Client;
use rpassword::prompt_password_stdout;
use rprompt::prompt_reply_stdout;
use serde::Serialize;
use structopt::{clap::AppSettings, StructOpt};
use walkdir::WalkDir;
use zip::CompressionMethod;

use std::{
    env::set_current_dir,
    fs::{self, File},
    io::Cursor,
    path::Path,
};

/// Factorio mod packager https://github.com/figsoda/pactorio
#[derive(StructOpt)]
#[structopt(name = "pactorio", global_setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Output info.json compactly
    #[structopt(short, long)]
    compact: bool,

    /// Specify the compression method, ignored without `-z/--zip` flag
    #[structopt(
        long,
        value_name = "METHOD",
        default_value = "stored",
        possible_values(&["stored", "bz2", "deflate"]),
        parse(from_str = compression_method),
    )]
    compression: CompressionMethod,

    /// Set working directory
    #[structopt(short, long, value_name = "DIRECTORY")]
    dir: Option<String>,

    /// Specify the config file to use
    #[structopt(short, long, value_name = "FILE", default_value = "pactorio.toml")]
    input: String,

    /// Specify the output directory
    #[structopt(short, long, value_name = "DIRECTORY", default_value = "release")]
    output: String,

    /// Publish to mod portal, accepts up to two arguments for username and password
    #[structopt(short, long, value_name = "CREDENTIAL", max_values = 2)]
    publish: Option<Vec<String>>,

    /// Output a zip file instead
    #[structopt(short, long)]
    zip: bool,
}

fn compression_method(compression: &str) -> CompressionMethod {
    match compression {
        "stored" => CompressionMethod::Stored,
        "bz2" => CompressionMethod::Bzip2,
        "deflate" => CompressionMethod::Deflated,
        _ => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();

    if let Some(dir) = opts.dir {
        set_current_dir(&dir).with_context(fail::set_dir(dir))?;
    }

    let cfg: Config =
        toml::from_str(&fs::read_to_string(&opts.input).with_context(fail::read(&opts.input))?)
            .with_context(fail::parse_cfg(&opts.input))?;

    let mut include = GlobSetBuilder::new();
    for pat in &cfg.source.include {
        include.add(Glob::new(pat).with_context(fail::parse_glob(pat))?);
    }
    let include = include.build().context("Failed to build glob set")?;

    let mut ignore = GlobSetBuilder::new();
    for pat in &cfg.source.ignore {
        ignore.add(Glob::new(pat).with_context(fail::parse_glob(pat))?);
    }
    let ignore = ignore.build().context("Failed to build glob set")?;

    let mut files = Vec::new();
    for entry in WalkDir::new(&cfg.source.dir).min_depth(1) {
        let entry = entry
            .with_context(fail::traverse(&cfg.source.dir))?
            .into_path();
        if include.is_match(&entry) && !ignore.is_match(&entry) {
            files.push((
                entry.clone(),
                entry
                    .strip_prefix(&cfg.source.dir)
                    .with_context(fail::traverse(&cfg.source.dir))?
                    .into(),
            ));
        }
    }

    let info = Info::from(cfg.clone());
    let info = if opts.compact {
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

    let file_name = &format!("{}_{}", cfg.package.name, cfg.package.version);
    if let Some(cred) = opts.publish {
        let mut zip = Cursor::new(Vec::with_capacity(256));
        release::zip(files, info, &mut zip, file_name.into(), opts.compression)?;

        if opts.zip {
            fs::create_dir_all(&opts.output).with_context(fail::create_dir(&opts.output))?;

            let output = &Path::new(&opts.output).join(format!("{}.zip", file_name));
            release::remove_path(output)?;

            let mut file =
                File::create(output).with_context(fail::create_file(output.display()))?;
            std::io::copy(&mut &zip.get_ref()[..], &mut file)
                .context("Failed to write to the zip file")?;
        }

        let mod_name = &cfg.package.name;
        let mod_version = &cfg.package.version;

        if publish::check_mod(mod_name, mod_version)
            .await
            .with_context(fail::query_mod(mod_name, mod_version))?
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

        let mut cred = cred.into_iter();
        let (username, password) = match cred.next() {
            Some(username) => (
                username,
                match cred.next() {
                    Some(password) => password,
                    None => prompt_password_stdout("Factorio password: ")
                        .context("Failed to prompt for password")?,
                },
            ),
            None => (
                prompt_reply_stdout("Factorio username: ")
                    .context("Failed to prompt for username")?,
                prompt_password_stdout("Factorio password: ")
                    .context("Failed to prompt for password")?,
            ),
        };

        publish::login(&client, csrf_token, username, password)
            .await
            .context("Failed to login to Factorio")?;

        let upload_token = publish::get_upload_token(&client, mod_name)
            .await
            .context("Failed to fetch upload token")?;

        publish::update_mod(&client, mod_name, upload_token, zip.into_inner())
            .await
            .with_context(fail::publish(mod_name, mod_version))?;

        if publish::check_mod(mod_name, mod_version)
            .await
            .with_context(fail::query_published(mod_name, mod_version))?
        {
            println!("{} v{} published successfully", mod_name, mod_version);
        } else {
            bail!("Failed to publish {}", mod_name);
        }
    } else if opts.zip {
        fs::create_dir_all(&opts.output).with_context(fail::create_dir(&opts.output))?;

        let output = &Path::new(&opts.output).join(format!("{}.zip", file_name));
        release::remove_path(output)?;

        let file = File::create(output).with_context(fail::create_file(output.display()))?;
        release::zip(files, info, file, file_name.into(), opts.compression)?;
    } else {
        let output = &Path::new(&opts.output).join(file_name);

        release::remove_path(output)?;
        fs::create_dir_all(output).with_context(fail::create_dir(output.display()))?;

        release::folder(files, info, output)?;
    }

    Ok(())
}
