use async_trait::async_trait;

use crate::domain::models::my_result::MyResult;

#[async_trait]
pub trait ResetService: Sync + Send {
    async fn reset_one_team(&self, id: i32) -> Result<MyResult, actix_web::Error>;
    async fn reset_all_teams(&self) -> Result<MyResult, actix_web::Error>;
    async fn reset_all_heroes(&self) -> Result<MyResult, actix_web::Error>;
}
