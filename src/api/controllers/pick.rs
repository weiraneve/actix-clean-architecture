use actix_web::{post, web, Responder};

use crate::app_state::AppState;
use crate::domain::models::post_param::PostParam;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(pick_heroes);
}

#[post("/")]
async fn pick_heroes(
    web::Json(param): web::Json<PostParam>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let response_data = app_state.service.pick.pick_heroes(param).await?;
    Ok(web::Json(response_data))
}
