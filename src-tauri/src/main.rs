// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn choose_rom() -> Result<String, ()> {
    Ok("Fake_ROM_File.sfc".to_string())
}

#[tauri::command]
async fn choose_patch() -> Result<String, ()> {
    Ok("Fake_Patch.ebp".to_string())
}

#[tauri::command]
async fn choose_destination() -> Result<String, ()> {
    Ok("Fake_ROM_File2.sfc".to_string())
}

#[tauri::command]
async fn patch_file() {
    println!("patching!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![choose_rom, choose_patch, choose_destination, patch_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
