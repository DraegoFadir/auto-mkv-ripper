use tauri_plugin_prevent_default::Flags;

mod settings;
mod tmdb;
mod tvdb;
mod makemkv;
mod sftp;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let prevent = if cfg!(debug_assertions) {
        tauri_plugin_prevent_default::Builder::new()
            .with_flags(Flags::all().difference(Flags::DEV_TOOLS))
            .build()
    } else {
        tauri_plugin_prevent_default::init()
    };

    tauri::Builder::default()
        .manage(tmdb::service::TMDBService::new(reqwest::Client::new()))
        .manage(tvdb::service::TVDBService::new(reqwest::Client::new()))
        .plugin(prevent)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tmdb::get_tmdb_by_id,
            tmdb::search_tmdb,
            tvdb::search_tvdb,
            makemkv::scan_disc,
            makemkv::rip_disc,
            sftp::send_sftp
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
