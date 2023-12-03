use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::domain::models::log::Log;

#[derive(Debug, Deserialize)]
pub struct MyResult {
    pub team_id: i32,
    pub data: String,
    pub time: NaiveDateTime,
    pub logs: Vec<Log>,
}
