use sqlx::{Pool, Sqlite};

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
