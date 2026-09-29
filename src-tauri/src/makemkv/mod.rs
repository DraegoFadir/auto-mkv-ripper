use std::{collections::BTreeMap, io::{BufRead, BufReader}, process::{Command, Stdio}};
use anyhow::Context;
use anyhow_tauri::{IntoTAResult, TAResult};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use ts_rs::TS;

use crate::settings::Settings;

#[derive(Default, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Title {
    index: u32,
    name: Option<String>,
    chapters: Option<u32>,
    duration: Option<String>,
    #[ts(type = "number | null")]
    size_bytes: Option<u64>,
    playlist: Option<String>,
    segment_map: Option<String>,
    file_name: Option<String>
}


#[derive(Deserialize)]
struct DiscInfoRow {
    title: u32,
    attr: u32,
    _code: u32,
    value: String
}

#[derive(Clone, serde::Serialize)]
struct RipProgress {
    current: u32,
    max: u32,
}

#[tauri::command]
pub async fn scan_disc(app: AppHandle) -> TAResult<Vec<Title>> {
    
    let settings: Settings = Settings::load(&app)?;

    // This is not necessary for linux which runs off flatpak
    #[cfg(target_os = "windows")] 
    if settings.makemkv_path.is_empty() {
        anyhow_tauri::bail!("MakeMKV Path is not set. Add it in Settings.");
    }

    let output = makemkvcon(settings.makemkv_path)
        .args(["-r", "--minlength=3600", "info", "disc:0"])
        .output()
        .into_ta_result()?;

    // This block was created with assistance of AI :(
    // I am a failure
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let tinfo: String = stdout.lines().filter_map(|x: &str| x.strip_prefix("TINFO:")).collect::<Vec<_>>().join("\n");
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(tinfo.as_bytes());

        let mut titles: BTreeMap<u32, Title> = BTreeMap::new();

        for row in reader.deserialize::<DiscInfoRow>() {
            let row: DiscInfoRow = row.into_ta_result()?;
            let t: &mut Title = titles
                .entry(row.title)
                .or_insert_with(|| Title {index: row.title, ..Default::default() });

            match row.attr {
                2 => t.name = Some(row.value),
                8 => t.chapters = row.value.parse().ok(),
                9 => t.duration = Some(row.value),
                11 => t.size_bytes = row.value.parse().ok(),
                16 => t.playlist = Some(row.value),
                26 => t.segment_map = Some(row.value),
                27 => t.file_name = Some(row.value),
                _ => {}
            }
        }

        Ok(titles.into_values().collect())
    }
}


#[tauri::command]
pub async fn rip_disc(title_index: u32, app: AppHandle) -> TAResult<()> {

    let settings: Settings = Settings::load(&app)?;

    #[cfg(target_os = "windows")]
    if settings.makemkv_path.is_empty() {
        anyhow_tauri::bail!("MakeMKV Path is not set. Add it in Settings.");
    }

    if settings.output_directory.is_empty() {
        anyhow_tauri::bail!("No Output Directory is set. Add it in Settings.");
    }

    let output = &settings.output_directory;

    std::fs::create_dir_all(output).into_ta_result()?;
    let mut child: std::process::Child = makemkvcon(settings.makemkv_path)
        .args(["-r", "--progress=-same", "--minlength=3600", "mkv", "disc:0", &(title_index.to_string()), output])
        .stdout(Stdio::piped())
        .spawn()
        .into_ta_result()?;

    let stdout = child.stdout.take().context("no stdout")?;

    // AI Helped with this too
    // I'm still learning the Rust, don't be too hard on me
    for line in BufReader::new(stdout).lines() {
        let line = line.into_ta_result()?;
        if let Some(rest) = line.strip_prefix("PRGV:") {
            let parts: Vec<u32> = rest.split(',').filter_map(|p| p.parse().ok()).collect();
            if let [current, _total, max] = parts[..] {
                app.emit("rip-progress", RipProgress { current, max }).ok();
            }
        } else if line.starts_with("MSG:") {
            println!("{line}")
        }
    }

    let status = child.wait().into_ta_result()?;
    if !status.success() {
        anyhow_tauri::bail!(format!("makemkvcon exited with {status}"));
    }

    Ok(())
}

// Function was AI Assisted
fn makemkvcon(_path: String) -> Command {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;

        let mut cmd: Command = Command::new(_path);
        cmd.creation_flags(0x08000000);
        cmd
    }
    
    #[cfg(target_os = "linux")]
    {
        let mut cmd: Command = Command::new("flatpak");
        cmd.args(["run", "--command=makemkvcon", "com.makemkv.MakeMKV"]);
        cmd
    }
}