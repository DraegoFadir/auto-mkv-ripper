mod settings;
mod tmdb;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tmdb::get_tmdb])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
