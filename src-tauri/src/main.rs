// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::CommandResult;
use models::DiabloItem;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::{fs, path::Path};
use tokio;

pub mod error;
pub mod models;
pub mod ocr;
pub mod queries;

#[tauri::command]
async fn screenshot_item(pool: tauri::State<'_, Pool<Sqlite>>) -> CommandResult<()> {
    let item = ocr::process_item()?;

    queries::insert_item(&pool, item.to_owned()).await?;

    Ok(())
}

#[tauri::command]
async fn remove_item(id: i64, pool: tauri::State<'_, Pool<Sqlite>>) -> CommandResult<()> {
    queries::remove_item(&pool, id).await?;

    Ok(())
}

#[tauri::command]
async fn get_all_items(pool: tauri::State<'_, Pool<Sqlite>>) -> CommandResult<Vec<DiabloItem>> {
    let items = queries::get_all_items(&pool).await?;

    Ok(items)
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
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            screenshot_item,
            remove_item,
            get_all_items
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
