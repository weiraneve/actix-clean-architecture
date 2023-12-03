use async_trait::async_trait;

use crate::domain::models::hero::Hero;

#[async_trait]
pub trait HeroRepository: Send + Sync {
    async fn get_not_is_pick(&self) -> Result<Vec<Hero>, sqlx::Error>;
    async fn save(&self, hero: Hero) -> Result<(), sqlx::Error>;
    async fn reset(&self) -> Result<(), sqlx::Error>;
}

