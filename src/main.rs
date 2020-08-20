mod types;
use types::{Config, Info};

use failure::Fallible;
use serde::Serialize;
use structopt::StructOpt;
use walkdir::WalkDir;

use std::{fs, path::Path};

/// Factorio mod packer
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
}

fn main() -> Fallible<()> {
    let opt = Opt::from_args();

    let cfg: Config = toml::from_str(&fs::read_to_string(opt.input)?)?;

    let mut files = Vec::new();
    for entry in WalkDir::new(&cfg.source.dir).min_depth(1) {
        files.push(entry?.path().to_owned());
    }

    let output = Path::new(&opt.output).join({
        let mut name = String::new();
        name.push_str(&cfg.package.name);
        name.push('_');
        name.push_str(&cfg.package.version);
        name
    });

    if output.is_dir() {
        fs::remove_dir_all(&output)?;
    } else if output.is_file() {
        fs::remove_file(&output)?;
    }
    fs::create_dir_all(&output)?;

    for from in files {
        if let Ok(to) = from.strip_prefix(&cfg.source.dir) {
            let to = output.join(to);
            if from.is_dir() {
                fs::create_dir(to)?;
            } else if from.is_file() {
                fs::copy(from, to)?;
            }
        }
    }

    let info = Info::from(cfg);
    fs::write(
        output.join("info.json"),
        if opt.compact {
            serde_json::to_string(&info)?
        } else {
            let mut writer = Vec::with_capacity(256);
            info.serialize(&mut serde_json::Serializer::with_formatter(
                &mut writer,
                serde_json::ser::PrettyFormatter::with_indent(b"    "),
            ))?;
            String::from_utf8(writer)?
        },
    )?;

    Ok(())
}
