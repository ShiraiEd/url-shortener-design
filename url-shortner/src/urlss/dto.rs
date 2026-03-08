use rapina::schemars::{self, JsonSchema};
use serde::Deserialize;
use rapina::sea_orm::prelude::*;

#[derive(Deserialize, JsonSchema)]
pub struct CreateUrls {
    pub long_url: String,
    pub created_at: DateTimeUtc,
    pub expires_at: DateTimeUtc,
    pub click_count: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdateUrls {
    pub short_code: Option<String>,
    pub long_url: Option<String>,
    pub created_at: Option<DateTimeUtc>,
    pub expires_at: Option<DateTimeUtc>,
    pub click_count: Option<i64>,
}
