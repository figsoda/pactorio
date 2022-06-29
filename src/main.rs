#![forbid(unsafe_code)]

mod cli;
mod fail;
mod publish;
mod release;
mod types;

use crate::{
    cli::Opts,
    types::{Config, Info},
};

use anyhow::{Context, Result};
use clap::Parser;
use globset::{Glob, GlobSetBuilder};
use rpassword::prompt_password;
use serde::Serialize;
use walkdir::WalkDir;

use std::{
    env::set_current_dir,
    fs::{self, File},
    io::Cursor,
    path::Path,
};

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(dir) = opts.dir {
        set_current_dir(&dir).with_context(fail::set_dir(dir.display()))?;
    }

    let cfg: Config = toml::from_str(
        &fs::read_to_string(&opts.input).with_context(fail::read(opts.input.display()))?,
    )
    .with_context(fail::parse_cfg(opts.input.display()))?;

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
    if let Some(api_key) = opts.publish {
        let mut zip = Cursor::new(Vec::with_capacity(256));
        release::zip(files, info, &mut zip, file_name.into(), opts.compression)?;

        if opts.zip {
            fs::create_dir_all(&opts.output)
                .with_context(fail::create_dir(opts.output.display()))?;

            let output = &Path::new(&opts.output).join(format!("{file_name}.zip"));
            release::remove_path(output)?;

            let mut file =
                File::create(output).with_context(fail::create_file(output.display()))?;
            std::io::copy(&mut &zip.get_ref()[..], &mut file)
                .context("Failed to write to the zip file")?;
        }

        let mod_name = &cfg.package.name;
        let mod_version = &cfg.package.version;

        let api_key = if let Some(x) = api_key {
            x
        } else {
            prompt_password("API key: ").context("Failed to prompt for api key")?
        };

        publish::upload_mod(mod_name, &api_key, &zip.into_inner())
            .with_context(fail::publish(mod_name, mod_version))?;
        eprintln!("{mod_name} version {mod_version} was uploaded successfully");
    } else if opts.zip {
        fs::create_dir_all(&opts.output).with_context(fail::create_dir(opts.output.display()))?;

        let output = &Path::new(&opts.output).join(format!("{file_name}.zip"));
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
