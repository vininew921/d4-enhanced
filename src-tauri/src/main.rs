// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::Path};

pub mod ocr;

#[tauri::command]
fn process_item() {
    ocr::process_item();
}

fn main() {
    setup_environment();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_environment() {
    if !Path::new("ocr_results").exists() {
        fs::create_dir("ocr_results").unwrap();
    }
}
