use std::sync::Arc;

use crate::domain::services::Service;
use crate::infrastructure::repositories::hero::HeroRepositoryImpl;
use crate::infrastructure::repositories::log::LogRepositoryImpl;
use crate::infrastructure::repositories::team::TeamRepositoryImpl;
use crate::services::pick::PickServiceImpl;
use crate::services::reset::ResetServiceImpl;
use sqlx::MySqlPool;

pub struct AppState {
    pub service: Arc<Service>,
}

impl AppState {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        let pick_service = Arc::new(PickServiceImpl::new(
            Arc::new(HeroRepositoryImpl::new(pool.clone())),
            Arc::new(TeamRepositoryImpl::new(pool.clone())),
            Arc::new(LogRepositoryImpl::new(pool.clone())),
        ));

        let reset_service = Arc::new(ResetServiceImpl::new(
            Arc::new(HeroRepositoryImpl::new(pool.clone())),
            Arc::new(TeamRepositoryImpl::new(pool.clone())),
        ));

        AppState {
            service: Arc::new(Service {
                pick: pick_service,
                reset: reset_service,
            }),
        }
    }
}
