use std::sync::Arc;

use anyhow_tauri::{IntoTAResult, TAResult};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use russh::keys::*;
use russh::*;
use russh_sftp::client::SftpSession;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use ts_rs::TS;
use tokio::fs::File as LocalFile;
use crate::settings::Settings;

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
pub struct Sftp {
    local_path: String,
    remote_path: String,
    file_path: String,
}

#[derive(Clone, serde::Serialize)]
struct SftpProgress {
    current: u64,
    max: u64,
}

#[tauri::command]
pub async fn send_sftp(app: AppHandle, sftp: Sftp)  -> TAResult<()> {
    let settings: Settings = Settings::load(&app)?;
    
    if settings.sftp_hostname.is_empty() {
        anyhow_tauri::bail!("No Sftp Hostname set. Set one in settings");
    }

    let config = russh::client::Config::default();
    let sh = Client {};
    let mut session = russh::client::connect(Arc::new(config), format!("{}:22", settings.sftp_hostname), sh)
        .await
        .into_ta_result()?;

    if settings.sftp_username.is_empty() {
        anyhow_tauri::bail!("No Sftp Username set. Set one in settings");
    }

    if settings.sftp_password.is_empty() {
        anyhow_tauri::bail!("No Sftp Password set. Set one in settings");
    }

    if session
        .authenticate_password(settings.sftp_username.trim(), settings.sftp_password.trim())
        .await
        .into_ta_result()?
        .success()
    {
        let channel = session.channel_open_session().await.into_ta_result()?;
        channel.request_subsystem(true, "sftp").await.into_ta_result()?;

        let sftp_session = SftpSession::new(channel.into_stream()).await.into_ta_result()?;

        let mut local = LocalFile::open(&sftp.local_path).await.into_ta_result()?;

        match sftp_session.create_dir(&sftp.remote_path).await {
            Ok(_) => {},
            Err(e) if e.to_string().contains("failure") => {},
            Err(e) => anyhow_tauri::bail!("[sftp.create_dir] error: {:?}", e.to_string()),
        }

        let mut remote = sftp_session
            .create(format!("{}/{}", &sftp.remote_path, &sftp.file_path))
            .await
            .into_ta_result()?;

        let total = local.metadata().await.into_ta_result()?.len();

        let mut buf = vec![0u8; 256 * 1024];
        let mut sent: u64 = 0;

        loop {
            let n = local.read(&mut buf).await.into_ta_result()?;
            if n == 0 { break; }
            remote.write_all(&buf[..n]).await.into_ta_result()?;
            sent += n as u64;
            println!("{} send of {} total", sent, total);
            app.emit("sftp-progress", SftpProgress { current: sent, max: total }).ok();
        }
    }

    Ok(())
}