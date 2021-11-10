use crate::types::{ModQuery, ModRelease, UploadResult};

use anyhow::{anyhow, bail, Result};
use multipart::client::lazy::Multipart;
use regex::Regex;
use ureq::Agent;

pub fn check_mod(mod_name: &str, mod_version: &str) -> Result<bool> {
    match ureq::get(&format!("https://mods.factorio.com/api/mods/{}", mod_name))
        .call()?
        .into_json()?
    {
        ModQuery::Err { message } => bail!(message),
        ModQuery::Mod { releases } => {
            for ModRelease { version } in releases {
                if mod_version == version {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

pub fn get_csrf_token(agent: &Agent) -> Result<String> {
    Ok(Regex::new(r#"csrf_token.*value="([^"]+)""#)?
        .captures(
            &agent
                .get("https://factorio.com/login?mods=1")
                .call()?
                .into_string()?,
        )
        .ok_or_else(|| anyhow!("Cannot find csrf token"))?[1]
        .into())
}

pub fn login(agent: &Agent, csrf_token: String, username: String, password: String) -> Result<()> {
    agent
        .post("https://factorio.com/login")
        .set("referer", "https://factorio.com/login?mods=1")
        .send_form(&[
            ("csrf_token", &csrf_token),
            ("username_or_email", &username),
            ("password", &password),
        ])?;

    Ok(())
}

pub fn get_upload_token(agent: &Agent, mod_name: &str) -> Result<String> {
    let upload_token = Regex::new("token: '(.*)'")?
        .captures(
            &agent
                .get(&format!(
                    "https://mods.factorio.com/mod/{}/downloads/edit",
                    mod_name,
                ))
                .call()?
                .into_string()?,
        )
        .ok_or_else(|| anyhow!("Cannot find a match with regex"))?[1]
        .into();
    Ok(upload_token)
}

pub fn update_mod(agent: &Agent, mod_name: &str, upload_token: String, file: &[u8]) -> Result<()> {
    let file_size = file.len();
    let parts = Multipart::new()
        .add_stream(
            "file",
            file,
            Some(format!("{}.zip", mod_name)),
            Some("application/x-zip-compressed".parse()?),
        )
        .prepare()?;
    let res = agent
        .post(&format!(
            "https://direct.mods-data.factorio.com/upload/mod/{}",
            upload_token
        ))
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", parts.boundary()),
        )
        .send(parts)?
        .into_json()?;
    agent
        .post(&format!(
            "https://mods.factorio.com/mod/{}/downloads/edit",
            mod_name,
        ))
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send(serde_urlencoded::to_string(UploadResult { file_size, ..res })?.as_bytes())?;

    Ok(())
}
