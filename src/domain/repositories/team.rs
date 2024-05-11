use crate::domain::models::team::Team;
use async_trait::async_trait;

#[async_trait]
pub trait TeamRepository: Send + Sync {
    async fn get_by_encrypt_code(&self, encrypt_code: String) -> Result<Team, sqlx::Error>;
    async fn save(&self, team: Team) -> Result<(), sqlx::Error>;
    async fn reset_one(&self, id: i32) -> Result<(), sqlx::Error>;

    async fn reset_all(&self) -> Result<(), sqlx::Error>;

    async fn get_by_id(&self, id: i32) -> Result<(), sqlx::Error>;
}
