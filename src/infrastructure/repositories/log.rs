use std::sync::Arc;

use crate::domain::models::log::Log;
use crate::domain::repositories::log::LogRepository;
use async_trait::async_trait;
use sqlx::MySqlPool;

pub struct LogRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl LogRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        LogRepositoryImpl { pool }
    }
}

#[async_trait]
impl LogRepository for LogRepositoryImpl {
    async fn get_by_team_id(&self, team_id: i32) -> Result<Vec<Log>, sqlx::Error> {
        sqlx::query_as::<_, Log>("SELECT * FROM `log` l WHERE l.team_id = ? ORDER BY l.time DESC")
            .bind(team_id.to_string())
            .fetch_all(&*self.pool)
            .await
    }

    async fn save(&self, log: Log) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO `log` (`team_id`, `pick_group`, `time`) VALUES (?, ?, ?)")
            .bind(log.team_id)
            .bind(log.pick_group)
            .bind(log.time)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
