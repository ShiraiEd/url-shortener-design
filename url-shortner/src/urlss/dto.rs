use rapina::{schemars::{self, JsonSchema}};
use serde::Deserialize;


#[derive(Deserialize, JsonSchema, validator::Validate)]
pub struct CreateUrls {
    #[validate(url)]
    pub long_url: String,
    pub expires_at: Option<String>,
}

