use clap::{builder::PossibleValue, Parser, ValueEnum};

use std::path::PathBuf;

/// Mod packager for Factorio
///
/// Homepage: https://github.com/figsoda/pactorio
#[derive(Parser)]
#[command(version)]
pub struct Opts {
    /// Output info.json compactly
    #[arg(short, long)]
    pub compact: bool,

    /// Output a zip file instead
    #[arg(short, long)]
    pub zip: bool,

    /// Specify the compression method, ignored without the `-z/--zip` flag
    #[arg(long, value_name = "method", default_value = "stored")]
    pub compression: CompressionMethod,

    /// Set working directory
    #[arg(short, long, value_name = "directory")]
    pub dir: Option<PathBuf>,

    /// Specify the config file to use
    #[arg(short, long, value_name = "file", default_value = "pactorio.toml")]
    pub input: PathBuf,

    /// Specify the output directory
    #[arg(short, long, value_name = "directory", default_value = "release")]
    pub output: PathBuf,

    // https://wiki.factorio.com/Mod_upload_API
    /// Upload to mod portal
    ///
    /// Requires an API key, which can be created on https://factorio.com/profile
    #[arg(short, long, value_name = "api-key")]
    pub upload: Option<Option<String>>,
}

#[derive(Clone)]
pub enum CompressionMethod {
    Stored,
    Bzip2,
    Deflated,
}

impl From<CompressionMethod> for zip::CompressionMethod {
    fn from(val: CompressionMethod) -> Self {
        match val {
            CompressionMethod::Stored => Self::Stored,
            CompressionMethod::Bzip2 => Self::Bzip2,
            CompressionMethod::Deflated => Self::Deflated,
        }
    }
}

impl ValueEnum for CompressionMethod {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Stored, Self::Bzip2, Self::Deflated]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::Stored => "stored",
            Self::Bzip2 => "bz2",
            Self::Deflated => "deflate",
        }))
    }
}
