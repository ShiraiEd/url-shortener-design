use rapina::prelude::*;
use rapina::database::{Db, DbError};
use rapina::sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set, ColumnTrait, QueryFilter};
use base62;
use rapina::response::BoxBody;

use crate::entity::Urls;
use crate::entity::urls::{ActiveModel, Model};

use super::dto::CreateUrls;
use super::error::UrlsError;

#[get("/", group = "/api/v1/shorten")]
#[errors(UrlsError)]
pub async fn list_urlss(db: Db) -> Result<Json<Vec<Model>>> {
    let items = Urls::find().all(db.conn()).await.map_err(DbError)?;
    Ok(Json(items))
}

#[get("/:code", group = "/api/v1/shorten")]
#[public]
#[cache(ttl = 300)]
#[errors(UrlsError)]
pub async fn redirect(db: Db, code: Path<String>) -> Result<http::Response<BoxBody>> {
    let code = code.into_inner();
    let item = Urls::find()
        .filter(crate::entity::urls::Column::ShortCode.eq(&code))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::not_found(format!("URL with code '{}' not found", code)))?;

    let now = rapina::sea_orm::prelude::DateTimeUtc::from(std::time::SystemTime::now());
    if item.expires_at < now {
        return Err(Error::new(410, "GONE", format!("URL '{}' has expired", code)));
    }

    let mut active: ActiveModel = item.clone().into_active_model();
    active.click_count = Set(item.click_count + 1);
    let _ = active.update(db.conn()).await.map_err(DbError)?;

    let response = http::Response::builder()
        .status(http::StatusCode::from_u16(302).unwrap())
        .header("Location", &item.long_url)
        .body(BoxBody::default())
        .unwrap();

    Ok(response)
}
#[public]
#[post("/", group = "/api/v1/shorten")]
#[errors(UrlsError)]
pub async fn create_urls(db: Db, body: Validated<Json<CreateUrls>>) -> Result<Json<serde_json::Value>> {
    let input = body.into_inner().into_inner();
    let item = ActiveModel {
        short_code: Set(String::new()),
        long_url: Set(input.long_url),
        created_at: Set(rapina::sea_orm::prelude::DateTimeUtc::from(std::time::SystemTime::now() + std::time::Duration::from_secs(9 * 3600))),
        expires_at: Set(input.expires_at
            .and_then(|s| s.parse::<rapina::sea_orm::prelude::DateTimeUtc>().ok())
            .unwrap_or_else(|| rapina::sea_orm::prelude::DateTimeUtc::from(std::time::SystemTime::now() + std::time::Duration::from_secs(24 * 365 * 3600)))),
        click_count: Set(0),
        ..Default::default()
    };
    let inserted = item.insert(db.conn()).await.map_err(DbError)?;

    let mut active: ActiveModel = inserted.into_active_model();
    active.short_code = Set(base62::encode(active.id.clone().unwrap() as u128 + 6767u128));
    let result = active.update(db.conn()).await.map_err(DbError)?;
    Ok(Json(serde_json::json!({"short_code": result.short_code, "long_url": result.long_url})))
}

#[public]
#[delete("/:short_code", group = "/api/v1/shorten")]
#[errors(UrlsError)]
pub async fn delete_code(db: Db, code: Path<String>) -> Result<Json<serde_json::Value>> {
    let code = code.into_inner();
    let result = Urls::delete_many()
        .filter(crate::entity::urls::Column::ShortCode.eq(&code))
        .exec(db.conn())
        .await
        .map_err(DbError)?;
    
    if result.rows_affected == 0 {
        return Err(Error::not_found(format!("Urls {} not found", code)));
    }
    Ok(Json(serde_json::json!({ "deleted": code })))
}
