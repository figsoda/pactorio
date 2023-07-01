use std::{error::Error, fmt::Write};

use anyhow::{anyhow, bail, Result};
use multipart::client::lazy::Multipart;
use ureq::Transport;

use crate::types::{FinishUploadResult, InitUploadResult};

fn transport_error(api_name: &str, e: Transport) -> Result<()> {
    let mut msg = format!("{api_name}: {}", e.kind());
    if let Some(message) = e.message() {
        write!(msg, ": {message}")?;
    }
    if let Some(source) = e.source() {
        write!(msg, ": {source}")?;
    }
    Err(anyhow!(msg))
}

pub fn upload_mod(mod_name: &str, api_key: &str, file: &[u8]) -> Result<()> {
    let upload_url = match ureq::post("https://mods.factorio.com/api/v2/mods/releases/init_upload")
        .set("authorization", &format!("Bearer {api_key}"))
        .send_form(&[("mod", mod_name)])
    {
        Err(ureq::Error::Status(code, res)) => {
            return Err(anyhow!("init_upload: status code {code}").context(
                match res.into_json()? {
                    InitUploadResult::Err(e) => e.to_string(),
                    _ => String::from("Unknown error"),
                },
            ))
        }
        Err(ureq::Error::Transport(e)) => return transport_error("init_upload", e),
        Ok(res) => match res.into_json()? {
            InitUploadResult::Err(e) => bail!(e),
            InitUploadResult::Ok { upload_url } => upload_url,
        },
    };

    let parts = Multipart::new()
        .add_stream(
            "file",
            file,
            Some(format!("{mod_name}.zip")),
            Some("application/x-zip-compressed".parse()?),
        )
        .prepare()?;

    match ureq::post(&upload_url)
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", parts.boundary()),
        )
        .send(parts)
    {
        Err(ureq::Error::Status(code, res)) => Err(anyhow!("finish_upload: status code {code}")
            .context(match res.into_json()? {
                FinishUploadResult::Err(e) => e.to_string(),
                _ => String::from("Unknown error"),
            })),
        Err(ureq::Error::Transport(e)) => transport_error("finish_upload", e),
        Ok(res) => match res.into_json()? {
            FinishUploadResult::Err(e) => Err(e.into()),
            _ => Ok(()),
        },
    }
}
