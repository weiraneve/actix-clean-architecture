use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Team {
    pub id: i32,
    pub encrypt_code: String,
    pub pick_content: String,
    pub is_picked: bool,
    pub update_time: NaiveDateTime,
}
