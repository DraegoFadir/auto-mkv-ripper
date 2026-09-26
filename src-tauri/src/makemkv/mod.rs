use std::process::Command;

use tauri::AppHandle;

#[tauri::command]
pub async fn scan_disc(app: AppHandle) -> Result<(), String> {
    let output = makemkvcon()
        .args(["-r", "info", "disc:0"])
        .output()
        .map_err(|e| e.to_string())?;

    println!("{:?}", output);
    Ok(())
}

fn makemkvcon() -> Command {
    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("flatpak");
        cmd.args([
            "run",
            "--command=makemkvcon",
            "com.makemkv.MakeMKV"
        ]);
        cmd
    }
}