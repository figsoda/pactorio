mod util;
use util::{get_info, Config};

use clap::clap_app;
use failure::Fallible;
use serde::Serialize;
use walkdir::WalkDir;

use std::{fs, path::Path};

fn app() -> Fallible<()> {
    let args = clap_app!(pactorio =>
        (version : "0.2.0")
        (author : "figsoda <figsoda@pm.me>")
        (about : "Factorio mod packer")
        (@arg compact : -c --compact "Output info.json compactly")
        (@arg input : -i --input [CONFIG_FILE] "Specify the config file to use")
        (@arg output : -o --output [OUTPUT_DIRECTORY] "Specify the output directory")
    )
    .get_matches();

    let cfg: Config = toml::from_str(&fs::read_to_string(
        args.value_of("input").unwrap_or("pactorio.toml"),
    )?)?;

    let mut files = Vec::new();
    for entry in WalkDir::new(&cfg.source.dir).min_depth(1) {
        files.push(entry?.path().to_owned());
    }

    let output = Path::new(args.value_of("output").unwrap_or("release")).join({
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

    let info = get_info(cfg);
    fs::write(
        output.join("info.json"),
        if args.is_present("compact") {
            serde_json::to_string(&info)?
        } else {
            let mut writer = Vec::with_capacity(256);
            let pretty = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(&mut writer, pretty);
            info.serialize(&mut ser)?;
            String::from_utf8(writer)?
        },
    )?;

    Ok(())
}

fn main() {
    let _ = app().map_err(|e| println!("{}", e));
}
