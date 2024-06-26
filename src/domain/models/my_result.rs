use crate::domain::models::log::Log;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResult {
    pub team_id: i32,
    pub data: String,
    pub time: NaiveDateTime,
    pub logs: Vec<Log>,
}
