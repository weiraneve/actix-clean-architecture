use actix_web::{get, Responder, web};
use crate::api::dto::team_query::TeamQuery;
use crate::app_state::AppState;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(reset_team);
    cfg.service(reset_all_teams);
    cfg.service(reset_all_heroes);
}

#[get("/reset/team")]
pub async fn reset_team(
    query: web::Query<TeamQuery>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let response_data = app_state.service.reset.reset_one_team(query.id).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/teams")]
pub async fn reset_all_teams(app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let response_data = app_state.service.reset.reset_all_teams().await?;
    Ok(web::Json(response_data))
}

#[get("/reset/heroes")]
pub async fn reset_all_heroes(app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let response_data = app_state.service.reset.reset_all_heroes().await?;
    Ok(web::Json(response_data))
}
