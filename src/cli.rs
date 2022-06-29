use clap::Parser;
use zip::CompressionMethod;

use std::path::PathBuf;

/// Mod packager for Factorio
///
/// Homepage: https://github.com/figsoda/pactorio
#[derive(Parser)]
#[clap(version)]
pub struct Opts {
    /// Output info.json compactly
    #[clap(short, long)]
    pub compact: bool,

    /// Output a zip file instead
    #[clap(short, long)]
    pub zip: bool,

    /// Specify the compression method, ignored without the `-z/--zip` flag
    #[clap(
        long,
        value_name = "method",
        default_value = "stored",
        possible_values = &["stored", "bz2", "deflate"],
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

    // https://wiki.factorio.com/Mod_upload_API
    /// Upload to mod portal
    ///
    /// Requires an API key, which can be created on https://factorio.com/profile
    #[clap(short, long, value_name = "api-key")]
    pub upload: Option<Option<String>>,
}

fn compression_method(compression: &str) -> CompressionMethod {
    match compression {
        "stored" => CompressionMethod::Stored,
        "bz2" => CompressionMethod::Bzip2,
        "deflate" => CompressionMethod::Deflated,
        _ => unreachable!(),
    }
}
