use std::sync::Arc;

use async_trait::async_trait;
use chrono::{FixedOffset, NaiveDateTime, Utc};

use crate::domain::models::my_result::MyResult;
use crate::domain::repositories::hero::HeroRepository;
use crate::domain::repositories::team::TeamRepository;
use crate::domain::services::reset::ResetService;

pub struct ResetServiceImpl {
    pub hero_repository: Arc<dyn HeroRepository>,
    pub team_repository: Arc<dyn TeamRepository>,
}

impl ResetServiceImpl {
    pub fn new(
        hero_repository: Arc<dyn HeroRepository>,
        team_repository: Arc<dyn TeamRepository>,
    ) -> Self {
        ResetServiceImpl {
            hero_repository,
            team_repository,
        }
    }

    fn create_result(data: String) -> MyResult {
        MyResult {
            team_id: 0,
            data,
            time: current_time(),
            logs: vec![],
        }
    }
}

#[async_trait]
impl ResetService for ResetServiceImpl {
    async fn reset_one_team(&self, id: i32) -> Result<MyResult, actix_web::Error> {
        match self.team_repository.get_by_id(id).await {
            Ok(()) => {
                self.team_repository
                    .reset_one(id)
                    .await
                    .expect(RESET_ONE_TEAM_FAILED_ERROR);
                Ok(Self::create_result(format!(
                    "{}{}{}",
                    RESET_ONE_TEAM_MESSAGE, id, RESET_ONE_TEAM_SUCCESS
                )))
            }
            Err(e) => {
                let message = match e {
                    sqlx::Error::RowNotFound => TEAM_NOT_FOUND_MESSAGE.to_string(),
                    _ => format!("{}: {}", ERROR_PROCESSING_MESSAGE, e),
                };
                Ok(Self::create_result(message))
            }
        }
    }

    async fn reset_all_teams(&self) -> Result<MyResult, actix_web::Error> {
        self.team_repository
            .reset_all()
            .await
            .expect(RESET_ALL_TEAMS_FAILED_ERROR);
        Ok(Self::create_result(
            RESET_ALL_TEAMS_SUCCESS_MESSAGE.to_string(),
        ))
    }

    async fn reset_all_heroes(&self) -> Result<MyResult, actix_web::Error> {
        self.hero_repository
            .reset()
            .await
            .expect(RESET_ALL_HEROES_FAILED_ERROR);
        Ok(Self::create_result(
            RESET_ALL_HEROES_SUCCESS_MESSAGE.to_string(),
        ))
    }
}

fn current_time() -> NaiveDateTime {
    Utc::now()
        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .naive_local()
}

const RESET_ONE_TEAM_FAILED_ERROR: &str = "reset one team failed";
const RESET_ALL_TEAMS_FAILED_ERROR: &str = "reset all teams failed";
const RESET_ALL_HEROES_FAILED_ERROR: &str = "reset all heroes failed";
const TEAM_NOT_FOUND_MESSAGE: &str = "未有查询到此队伍";
const ERROR_PROCESSING_MESSAGE: &str = "处理时遇到错误";
const RESET_ONE_TEAM_MESSAGE: &str = "刷新队伍";
const RESET_ONE_TEAM_SUCCESS: &str = "成功";
const RESET_ALL_TEAMS_SUCCESS_MESSAGE: &str = "重置所有队伍成功";
const RESET_ALL_HEROES_SUCCESS_MESSAGE: &str = "重置所有英雄成功";
