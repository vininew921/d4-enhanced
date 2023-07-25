// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::{fs, path::Path};
use tokio;

pub mod models;
pub mod ocr;
pub mod queries;

#[tauri::command]
fn process_item() {
    if let Err(x) = ocr::process_item() {
        println!("process_item error: {}", x.to_string());
    }
}

#[tokio::main]
async fn main() {
    if !Path::new("ocr_results").exists() {
        fs::create_dir("ocr_results").unwrap();
    }

    let db_url = "sqlite://database.db";

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);

        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Success creating db"),
            Err(error) => panic!("Error: {}", error),
        }
    } else {
        println!("Database found");
    }

    let db = SqlitePool::connect(db_url).await.unwrap();

    queries::setup_db(&db).await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
