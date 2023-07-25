use crate::models::DiabloItem;
use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};

pub async fn setup_db(pool: &Pool<Sqlite>) {
    let query = r#"
    CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text_value TEXT,
        image_id TEXT,
        created_on TIMESTAMP DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'utc'))
    );
    "#;

    let result = sqlx::query(query).execute(pool).await.unwrap();
    println!("Setup_DB result: {:?}", result);
}

pub async fn insert_item(
    pool: &Pool<Sqlite>,
    item: DiabloItem,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO items (text_value, image_id) VALUES (?, ?)")
        .bind(item.text_value)
        .bind(item.image_id)
        .execute(pool)
        .await
}

pub async fn remove_item(pool: &Pool<Sqlite>, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM items WHERE id=?")
        .bind(id)
        .execute(pool)
        .await
}

pub async fn get_all_items(pool: &Pool<Sqlite>) -> Result<Vec<DiabloItem>, sqlx::Error> {
    sqlx::query_as::<_, DiabloItem>("SELECT * FROM items")
        .fetch_all(pool)
        .await
}
