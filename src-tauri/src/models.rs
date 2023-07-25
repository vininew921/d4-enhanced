use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct DiabloItem {
    pub id: i64,
    pub text_value: String,
    pub image_id: String,
    pub created_on: DateTime<Utc>,
}

impl DiabloItem {
    pub fn new(text_value: String, image_id: String) -> Self {
        DiabloItem {
            id: 0,
            text_value,
            image_id,
            created_on: Utc::now(),
        }
    }
}
