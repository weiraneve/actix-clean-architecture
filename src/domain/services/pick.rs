use crate::domain::models::my_result::MyResult;
use crate::domain::models::post_param::PostParam;
use async_trait::async_trait;

#[async_trait]
pub trait PickService: Sync + Send {
    async fn pick_heroes(&self, param: PostParam) -> Result<MyResult, actix_web::Error>;
}
