use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Log {
    pub team_id: i32,
    #[serde(rename = "pickGroup")]
    pub pick_group: String,
    pub time: NaiveDateTime,
}
