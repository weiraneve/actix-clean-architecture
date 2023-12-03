use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostParam {
    #[serde(rename = "encryptCode")]
    pub encrypt_code: String,
}
