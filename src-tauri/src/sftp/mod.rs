use std::sync::Arc;

use anyhow::Context;
use anyhow_tauri::{IntoTAResult, TAResult};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use russh::keys::*;
use russh::*;
use russh_sftp::client::SftpSession;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use ts_rs::TS;
use tokio::fs::File as LocalFile;
use crate::{models::{Status, TitleProgress, TitleStatus}, settings::{self, Settings}};

struct Client;

impl russh::client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MediaType {
    #[default]
    Movie, 
    Series, 
    Anime
}

#[derive(Default, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Sftp {
    local_path: String,
    remote_path: String,
    file_path: String,
    media_type: MediaType,
    #[ts(optional)]
    split_path: Option<String>,
}

#[tauri::command]
pub async fn send_sftp(app: AppHandle, title_index: u32, sftp: Sftp)  -> TAResult<()> {
    let settings: Settings = Settings::load(&app)?;
    let sftp_host = settings.sftp_hostname()?;
    let sftp_username = settings.sftp_username()?;
    let sftp_password = settings.sftp_password()?;

    let config = russh::client::Config::default();
    let sh = Client {};
    let mut session = russh::client::connect(Arc::new(config), format!("{}:22", sftp_host), sh)
        .await
        .into_ta_result()?;

    if session
        .authenticate_password(sftp_username.trim(), sftp_password.trim())
        .await
        .into_ta_result()?
        .success()
    {
        let channel = session.channel_open_session().await.into_ta_result()?;
        channel.request_subsystem(true, "sftp").await.into_ta_result()?;
        
        let sftp_session = SftpSession::new(channel.into_stream()).await.into_ta_result()?;
        
        app.emit("sftp-status", TitleStatus { title_index, status: Status::Started }).ok();

        let result = run_sftp(&app, title_index, &sftp, sftp_session).await;
        let status = if result.is_ok() { Status::Done } else { Status::Failed };
        
        app.emit("sftp-status", TitleStatus { title_index, status }).ok();
        result
    } else {
        anyhow_tauri::bail!("SFTP authentication failed. Check your username and password.");
    }
}

async fn run_sftp(app: &AppHandle, title_index: u32, sftp: &Sftp, sftp_session: SftpSession) -> TAResult<()> {

    let settings: Settings = Settings::load(&app)?;
    
    let mut local = LocalFile::open(&sftp.local_path).await.into_ta_result()?;
    let total = local.metadata().await.into_ta_result()?.len();
    if total == 0 {
        anyhow_tauri::bail!("File is empty");
    }


    let mut remote: String;
    match sftp.media_type {
        MediaType::Movie => remote = settings.sftp_movie_path()?,
        MediaType::Series => remote = settings.sftp_tvshow_path()?,
        MediaType::Anime => remote = settings.sftp_anime_path()?,
        _ => anyhow_tauri::bail!("invalid media_type"),
    }

    
    // If you find this setting, cool
    // This is not going to be officially documented but was made to fit how i upload
    if remote.contains("~split") {
        if let Some(value) = &sftp.split_path {
            remote = remote.replace("~split", &value);
        }
    }

    let mut path: String = remote.into();
    let parts = sftp.remote_path.split("/");
    for part in parts {
        if !path.is_empty() {
            path.push('/');
        }

        path.push_str(part);

        if !sftp_session.try_exists(&path).await.into_ta_result()? {
            sftp_session.create_dir(&path).await
                .with_context(|| format!("Failed to create the remote directory: {path}{}", sftp.remote_path))
                .into_ta_result()?;
        }
    }
    

    let mut remote = sftp_session
        .create(format!("{}/{}", &path, &sftp.file_path))
        .await
        .into_ta_result()?;

    let mut buf = vec![0u8; 256 * 1024];
    let mut sent: u64 = 0;

    let mut last_pct = 0;

    loop {
        let n = local.read(&mut buf).await.into_ta_result()?;
        if n == 0 { break; }
        remote.write_all(&buf[..n]).await.into_ta_result()?;
        sent += n as u64;

        let curr_pct = sent * 100 / total;
        if curr_pct > last_pct {
            println!("{} send of {} total", sent, total);
            app.emit("sftp-progress", TitleProgress { title_index, total: None, current: sent, max: total }).ok();
            last_pct = curr_pct;
        }
    }

    remote.shutdown().await.into_ta_result()?;
    Ok(())
}