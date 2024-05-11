use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::domain::models::team::Team;
use crate::domain::repositories::team::TeamRepository;

pub struct TeamRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl TeamRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        TeamRepositoryImpl { pool }
    }
}

#[async_trait]
impl TeamRepository for TeamRepositoryImpl {
    async fn get_by_encrypt_code(&self, encrypt_code: String) -> Result<Team, sqlx::Error> {
        sqlx::query_as::<_, Team>("SELECT * FROM `team` WHERE `encrypt_code` = ?")
            .bind(encrypt_code)
            .fetch_one(&*self.pool)
            .await
    }

    async fn save(&self, team: Team) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        INSERT INTO `team` (`id`, `encrypt_code`, `pick_content`, `is_picked`, `update_time`)
        VALUES (?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            `encrypt_code` = VALUES(`encrypt_code`),
            `pick_content` = VALUES(`pick_content`),
            `is_picked` = VALUES(`is_picked`),
            `update_time` = VALUES(`update_time`)
    "#,
        )
        .bind(team.id)
        .bind(team.encrypt_code)
        .bind(team.pick_content)
        .bind(team.is_picked)
        .bind(team.update_time)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn reset_one(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE `team` SET `pick_content`='',`is_picked`=0,`update_time`=current_time WHERE `id`=(?)")
            .bind(id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn reset_all(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE `team` SET `pick_content`='',`is_picked`=0,`update_time`=current_time WHERE `is_picked`=true")
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query_as::<_, Team>("SELECT * FROM `team` h WHERE h.id = (?)")
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(())
    }
}
