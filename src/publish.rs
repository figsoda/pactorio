use crate::types::{ModQuery, ModRelease, UploadResult};

use anyhow::{anyhow, bail, Result};
use regex::Regex;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use rpassword::prompt_password_stdout;
use rprompt::prompt_reply_stdout;
use select::{document::Document, predicate::Attr};

use std::fmt::Display;

pub async fn check_mod(
    mod_name: impl Display,
    mod_version: &impl PartialEq<String>,
) -> Result<bool> {
    match reqwest::get(&format!("https://mods.factorio.com/api/mods/{}", mod_name))
        .await?
        .json()
        .await?
    {
        ModQuery::Err { message } => bail!(message),
        ModQuery::Mod { releases } => {
            for ModRelease { version } in releases {
                if mod_version == &version {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

pub async fn get_csrf_token(client: &Client) -> Result<String> {
    let doc: Document = client
        .get("https://factorio.com/login?mods=1")
        .send()
        .await?
        .text()
        .await?
        .as_str()
        .into();

    let csrf_token = doc
        .find(Attr("id", "csrf_token"))
        .next()
        .ok_or(anyhow!("Cannot find node with id=\"csrf_token\""))?
        .attr("value")
        .ok_or(anyhow!("Node does not contain attribute named \"value\""))?
        .into();

    Ok(csrf_token)
}

pub async fn login(client: &Client, csrf_token: String) -> Result<()> {
    client
        .post("https://factorio.com/login?mods=1")
        .header("referer", "https://factorio.com/login")
        .form(&[
            ("csrf_token", &csrf_token),
            ("username_or_email", &prompt_reply_stdout("Username: ")?),
            ("password", &prompt_password_stdout("Password: ")?),
        ])
        .send()
        .await?;

    Ok(())
}

pub async fn get_upload_token(client: &Client, mod_name: impl Display) -> Result<String> {
    let upload_token = Regex::new("token: '(.*)'")?
        .captures(
            &client
                .get(&format!(
                    "https://mods.factorio.com/mod/{}/downloads/edit",
                    mod_name,
                ))
                .send()
                .await?
                .text()
                .await?,
        )
        .ok_or(anyhow!("Cannot find a match with regex"))?[1]
        .into();
    Ok(upload_token)
}

pub async fn update_mod(
    client: &Client,
    mod_name: impl Display,
    upload_token: impl Display,
    file: Vec<u8>,
) -> Result<()> {
    let file_size = file.len();
    let res = client
        .post(&format!(
            "https://direct.mods-data.factorio.com/upload/mod/{}",
            upload_token
        ))
        .multipart(
            Form::new().part(
                "file",
                Part::bytes(file)
                    .file_name(format!("{}.zip", mod_name))
                    .mime_str("application/x-zip-compressed")?,
            ),
        )
        .send()
        .await?
        .json()
        .await?;
    client
        .post(&format!(
            "https://mods.factorio.com/mod/{}/downloads/edit",
            mod_name,
        ))
        .form(&UploadResult { file_size, ..res })
        .send()
        .await?;

    Ok(())
}
