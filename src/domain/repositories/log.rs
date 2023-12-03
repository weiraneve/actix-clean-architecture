use async_trait::async_trait;
use crate::domain::models::log::Log;

#[async_trait]
pub trait LogRepository: Send + Sync {
    async fn get_by_team_id(&self, team_id: i32) -> Result<Vec<Log>, sqlx::Error>;
    async fn save(&self, log: Log) -> Result<(), sqlx::Error>;
}
