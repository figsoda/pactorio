use clap::clap_app;
use failure::Fallible;
use serde::{ Deserialize, Serialize };
use walkdir::WalkDir;

use std::{
    collections::HashMap,
    default::Default,
    env,
    fs,
    option::Option,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(into = "Vec<String>")]
struct Deps {
    #[serde(default)]
    required : HashMap<String, String>,
    #[serde(default)]
    #[serde(alias = "conflict")]
    incompatible : HashMap<String, String>,
    #[serde(default)]
    optional : HashMap<String, String>,
    #[serde(default)]
    hidden : HashMap<String, String>,
}

impl Into<Vec<String>> for Deps {
    fn into(self) -> Vec<String> {
        let mut deps = Vec::new();

        for (name, version) in self.required {
            deps.push(format!("{} {}", name, version));
        }

        for (name, version) in self.incompatible {
            deps.push(format!("! {} {}", name, version));
        }

        for (name, version) in self.optional {
            deps.push(format!("? {} {}", name, version));
        }

        for (name, version) in self.hidden {
            deps.push(format!("(?) {} {}", name, version));
        }

        deps
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Package {
    name : String,
    version : String,
    title : String,
    author : String,
    contact : Option<String>,
    homepage : Option<String>,
    description : Option<String>,
    factorio_version : Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct Source {
    directory : String,
}

impl Default for Source {
    fn default() -> Source {
        Source {
            directory : String::from("src"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Config {
    package : Package,
    #[serde(alias = "dependencies")]
    deps : Option<Deps>,
    #[serde(default)]
    source : Source,
}

#[derive(Clone, Debug, Serialize)]
struct Info {
    name : String,
    version : String,
    title : String,
    author : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    factorio_version : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies : Option<Deps>
}

pub fn app() -> Fallible<()> {
    let args = clap_app!(pactorio =>
        (version : "0.1.0")
        (author : "figsoda <figsoda@pm.me>")
        (about : "Package factorio mods")
        (@arg compact : -c --compact "Outputs info.json compactly")
        (@arg input : -i --input [PACTORIO_FILE] "Sets the pactorio file to use")
    ).get_matches();

    let cfg : Config = toml::from_str(
        &fs::read_to_string(args
            .value_of("input")
            .unwrap_or("pactorio.toml")
        )?,
    )?;

    let mut files = Vec::new();
    for entry in WalkDir::new(&cfg.source.directory).min_depth(1) {
        files.push(entry?.path().to_owned());
    }

    let info = Info {
        name : cfg.package.name.clone(),
        version : cfg.package.version.clone(),
        title : cfg.package.title.clone(),
        author : cfg.package.author.clone(),
        contact : cfg.package.contact.clone(),
        homepage : cfg.package.homepage.clone(),
        description : cfg.package.description.clone(),
        factorio_version : cfg.package.factorio_version.clone(),
        dependencies : cfg.deps.clone(),
    };

    let here = env::current_dir()?;
    let mut output = String::new();
    output.push_str(&cfg.package.name);
    output.push('_');
    output.push_str(&cfg.package.version);
    let output = here.join(output);

    if output.is_dir() {
        fs::remove_dir_all(&output)?;
    } else if output.is_file() {
        fs::remove_file(&output)?;
    }
    fs::create_dir(&output)?;

    for from in files {
        if let Ok(to) = from.strip_prefix(&cfg.source.directory) {
            let to = output.join(to);
            if from.is_dir() {
                fs::create_dir(to)?;
            } else if from.is_file() {
                fs::copy(from, to)?;
            }
        }
    }

    fs::write(output.join("info.json"), if args.is_present("compact") {
        serde_json::to_string(&info)?
    } else {
        let mut writer = Vec::with_capacity(256);
        let pretty = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut writer, pretty);
        info.serialize(&mut ser)?;
        String::from_utf8(writer)?
    })?;

    Ok(())
}
