use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub line: i32,
    pub is_pick: bool,
}
