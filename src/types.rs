use serde::{Deserialize, Serialize};

use std::{collections::HashMap, default::Default};

#[derive(Clone, Serialize, Deserialize)]
pub struct Deps {
    #[serde(default)]
    required: HashMap<String, String>,
    #[serde(alias = "incompatible")]
    #[serde(default)]
    conflict: HashMap<String, String>,
    #[serde(default)]
    optional: HashMap<String, String>,
    #[serde(default)]
    hidden: HashMap<String, String>,
}

#[derive(Clone, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub title: String,
    pub author: String,
    pub contact: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,
    pub factorio_version: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct Source {
    #[serde(alias = "directory")]
    #[serde(default = "default_source_dir")]
    pub dir: String,
    #[serde(default = "default_source_include")]
    pub include: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

fn default_source_dir() -> String {
    String::from("src")
}

fn default_source_include() -> Vec<String> {
    vec![String::from("**/**")]
}

impl Default for Source {
    fn default() -> Source {
        Source {
            dir: default_source_dir(),
            include: default_source_include(),
            ignore: Vec::new(),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub package: Package,
    #[serde(alias = "dependencies")]
    pub deps: Option<Deps>,
    #[serde(default)]
    pub source: Source,
}

#[derive(Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub title: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factorio_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
}

impl From<Config> for Info {
    fn from(cfg: Config) -> Self {
        Info {
            name: cfg.package.name,
            version: cfg.package.version,
            title: cfg.package.title,
            author: cfg.package.author,
            contact: cfg.package.contact,
            homepage: cfg.package.homepage,
            description: cfg.package.description,
            factorio_version: cfg.package.factorio_version,
            dependencies: cfg.deps.map(|deps| {
                let mut xs = Vec::new();

                for (name, version) in deps.required {
                    xs.push(format!("{} {}", name, version));
                }

                for (name, version) in deps.conflict {
                    xs.push(format!("! {} {}", name, version));
                }

                for (name, version) in deps.optional {
                    xs.push(format!("? {} {}", name, version));
                }

                for (name, version) in deps.hidden {
                    xs.push(format!("(?) {} {}", name, version));
                }

                xs
            }),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UploadResult {
    pub changelog: Option<String>,
    pub filename: String,
    #[serde(skip_deserializing)]
    pub file_size: usize,
    #[serde(rename(serialize = "info_json"))]
    pub info: String,
    pub thumbnail: Option<String>,
}

#[derive(Deserialize)]
pub struct ModRelease {
    pub version: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ModQuery {
    Err { message: String },
    Mod { releases: Vec<ModRelease> },
}
