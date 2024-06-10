use std::sync::Arc;

use crate::domain::models::hero::Hero;
use crate::domain::models::log::Log;
use crate::domain::models::my_result::MyResult;
use crate::domain::models::post_param::PostParam;
use crate::domain::models::team::Team;
use crate::domain::repositories::hero::HeroRepository;
use crate::domain::repositories::log::LogRepository;
use crate::domain::repositories::team::TeamRepository;
use crate::domain::services::pick::PickService;
use async_trait::async_trait;
use chrono::{FixedOffset, NaiveDateTime, Utc};

pub struct PickServiceImpl {
    pub hero_repository: Arc<dyn HeroRepository>,
    pub team_repository: Arc<dyn TeamRepository>,
    pub log_repository: Arc<dyn LogRepository>,
}

impl PickServiceImpl {
    pub fn new(
        hero_repository: Arc<dyn HeroRepository>,
        team_repository: Arc<dyn TeamRepository>,
        log_repository: Arc<dyn LogRepository>,
    ) -> Self {
        PickServiceImpl {
            hero_repository,
            team_repository,
            log_repository,
        }
    }
}

#[async_trait]
impl PickService for PickServiceImpl {
    async fn pick_heroes(&self, param: PostParam) -> Result<MyResult, actix_web::Error> {
        let mut team = self
            .team_repository
            .get_by_encrypt_code(param.encrypt_code)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let mut result = MyResult {
            team_id: team.id,
            data: team.pick_content.clone(),
            time: current_time(),
            logs: self
                .log_repository
                .get_by_team_id(team.id)
                .await
                .expect(GET_LOGS_FAILED_ERROR),
        };

        self.check_team_is_picked(&mut team, &mut result).await;
        Ok(result)
    }
}

impl PickServiceImpl {
    async fn check_team_is_picked(&self, team: &mut Team, result: &mut MyResult) {
        if !team.is_picked {
            let pick_heroes = self
                .hero_repository
                .get_not_is_pick()
                .await
                .expect(PICK_HEROES_FAILED_ERROR);

            let pick_result = self.get_pick_result(pick_heroes).await;
            result.data.clone_from(&pick_result);

            self.save_result_for_log(team.id, &pick_result).await;
            self.update_team_is_picked(team, &pick_result).await;
        }
    }

    async fn get_pick_result(&self, mut heroes: Vec<Hero>) -> String {
        for hero in &mut heroes {
            hero.is_pick = true;
            self.hero_repository
                .save(hero.clone())
                .await
                .expect(SAVE_HERO_FAILED_ERROR);
        }

        let names: Vec<String> = heroes.into_iter().map(|hero| hero.name).collect();
        format!(
            "[{}]or[{}]",
            names[..HEROES_AMOUNT / 2].join(","),
            names[HEROES_AMOUNT / 2..].join(",")
        )
    }

    async fn update_team_is_picked(&self, team: &mut Team, pick_result: &str) {
        team.is_picked = true;
        pick_result.clone_into(&mut team.pick_content);
        team.update_time = current_time();
        self.team_repository
            .save(team.clone())
            .await
            .expect(SAVE_TEAM_FAILED_ERROR);
    }

    async fn save_result_for_log(&self, team_index: i32, pick_result: &str) {
        let log = Log {
            team_id: team_index,
            pick_group: pick_result.to_owned(),
            time: current_time(),
        };
        self.log_repository
            .save(log)
            .await
            .expect(SAVE_LOG_FAILED_ERROR);
    }
}

fn current_time() -> NaiveDateTime {
    Utc::now()
        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .naive_local()
}

const GET_LOGS_FAILED_ERROR: &str = "Failed to get logs";
const PICK_HEROES_FAILED_ERROR: &str = "pick heroes failed";
const SAVE_HERO_FAILED_ERROR: &str = "save hero failed";
const SAVE_TEAM_FAILED_ERROR: &str = "save team failed";
const SAVE_LOG_FAILED_ERROR: &str = "save log failed";
const HEROES_AMOUNT: usize = 4;
