use std::sync::Arc;
use crate::domain::services::pick::PickService;
use crate::domain::services::reset::ResetService;

pub mod pick;
pub mod reset;

pub struct Service {
    pub pick: Arc<dyn PickService>,
    pub reset: Arc<dyn ResetService>,
}
