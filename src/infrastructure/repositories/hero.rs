use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;
use crate::domain::models::hero::Hero;
use crate::domain::repositories::hero::HeroRepository;


pub struct HeroRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl HeroRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        HeroRepositoryImpl { pool }
    }
}

#[async_trait]
impl HeroRepository for HeroRepositoryImpl {
    async fn get_not_is_pick(&self) -> Result<Vec<Hero>, sqlx::Error> {
        sqlx::query_as::<_, Hero>("SELECT * FROM `hero` h WHERE h.is_pick = FALSE ORDER BY RAND() LIMIT 4")
            .fetch_all(&*self.pool)
            .await
    }

    async fn save(&self, hero: Hero) -> Result<(), sqlx::Error> {
        sqlx::query(r#"INSERT INTO hero (id, name, line, is_pick)
        VALUES (?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
        name = VALUES(name),
        line = VALUES(line),
        is_pick = VALUES(is_pick)
    "#)
            .bind(hero.id)
            .bind(&hero.name)
            .bind(&hero.line)
            .bind(hero.is_pick)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn reset(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE `hero` SET `is_pick`=false WHERE `is_pick`=true ")
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
