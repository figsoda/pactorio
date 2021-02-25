use clap::{AppSettings, Clap};
use zip::CompressionMethod;

use std::path::PathBuf;

/// Factorio mod packager https://github.com/figsoda/pactorio
#[derive(Clap)]
#[clap(bin_name = "pactorio", version, global_setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Output info.json compactly
    #[clap(short, long)]
    pub compact: bool,

    /// Output a zip file instead
    #[clap(short, long)]
    pub zip: bool,

    /// Specify the compression method, ignored without `-z/--zip` flag
    #[clap(
        long,
        value_name = "method",
        default_value = "stored",
        possible_values(&["stored", "bz2", "deflate"]),
        parse(from_str = compression_method),
    )]
    pub compression: CompressionMethod,

    /// Set working directory
    #[clap(short, long, value_name = "directory")]
    pub dir: Option<PathBuf>,

    /// Specify the config file to use
    #[clap(short, long, value_name = "file", default_value = "pactorio.toml")]
    pub input: PathBuf,

    /// Specify the output directory
    #[clap(short, long, value_name = "directory", default_value = "release")]
    pub output: PathBuf,

    /// Publish to mod portal, accepts up to two arguments for username and password
    #[clap(short, long, value_name = "credential", max_values = 2)]
    pub publish: Option<Vec<String>>,
}

pub fn compression_method(compression: &str) -> CompressionMethod {
    match compression {
        "stored" => CompressionMethod::Stored,
        "bz2" => CompressionMethod::Bzip2,
        "deflate" => CompressionMethod::Deflated,
        _ => unreachable!(),
    }
}
