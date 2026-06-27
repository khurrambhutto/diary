//#[tauri::command]
//fn greet(name: &str) -> String {
//    format!("Hello, {}! You've been greeted from Rust!", name)
//}
//
// Phase 1 has zero Tauri commands — persistence is handled purely on the JS
// side via @tauri-apps/plugin-store. When the first genuine command lands
// (e.g. file dialogs, export), create src/commands.rs and register it here
// with `mod commands;` + tauri::generate_handler![commands::...].

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}