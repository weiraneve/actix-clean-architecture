use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamQuery {
    pub(crate) id: i32,
}
