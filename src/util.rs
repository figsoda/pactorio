use serde::{Deserialize, Serialize};

use std::{collections::HashMap, default::Default, option::Option};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Source {
    #[serde(alias = "directory")]
    pub dir: String,
}

impl Default for Source {
    fn default() -> Source {
        Source {
            dir: String::from("src"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub package: Package,
    #[serde(alias = "dependencies")]
    pub deps: Option<Deps>,
    #[serde(default)]
    pub source: Source,
}

#[derive(Clone, Debug, Serialize)]
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

pub fn get_info(cfg: Config) -> Info {
    Info {
        name: cfg.package.name.clone(),
        version: cfg.package.version.clone(),
        title: cfg.package.title.clone(),
        author: cfg.package.author.clone(),
        contact: cfg.package.contact.clone(),
        homepage: cfg.package.homepage.clone(),
        description: cfg.package.description.clone(),
        factorio_version: cfg.package.factorio_version.clone(),
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
