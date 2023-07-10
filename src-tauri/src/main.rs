// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn patch_file(rom_file: String, patch_file: String, destination_file: String) {
    println!("patching!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![patch_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
