use rapina::cache::CacheConfig;
use rapina::database::DatabaseConfig;
use rapina::middleware::RequestLogMiddleware;
use rapina::prelude::*;
use rapina::schemars;

use crate::urlss::handlers::{create_urls, delete_code, list_urlss, redirect};

mod entity;
mod migrations;
mod urlss;

#[derive(Serialize, JsonSchema)]
struct MessageResponse {
    message: String,
}

#[derive(Serialize, JsonSchema)]
struct HealthResponse {
    status: String,
    version: String,
}

#[get("/")]
async fn hello() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Hello from Rapina!".to_string(),
    })
}

#[get("/health")]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::new()
        .get("/", hello)
        .get("/health", health)
        .get("/api/v1/shorten", list_urlss)
        .post("/api/v1/shorten", create_urls)
        .get("/api/v1/shorten/:short_code", redirect)
        .delete("/api/v1/shorten/:short_code", delete_code);

    Rapina::new()
        .with_tracing(TracingConfig::new())
        .with_rate_limit(RateLimitConfig::per_minute(60))
        .with_cache(CacheConfig::in_memory(10_000)).await?
        .middleware(RequestLogMiddleware::new())
        .with_database(DatabaseConfig::new("sqlite://urls.db?mode=rwc"))
        .await?
        .run_migrations::<migrations::Migrator>()
        .await?
        .router(router)
        .listen("127.0.0.1:3000")
        .await
}
