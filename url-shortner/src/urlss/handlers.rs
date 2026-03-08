use rapina::prelude::*;
use rapina::database::{Db, DbError};
use rapina::sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set, ColumnTrait, QueryFilter};
use base62;

use crate::entity::Urls;
use crate::entity::urls::{ActiveModel, Model};

use super::dto::{CreateUrls, UpdateUrls};
use super::error::UrlsError;

#[get("/", group = "/api/v1/shorten")]
#[errors(UrlsError)]
pub async fn list_urlss(db: Db) -> Result<Json<Vec<Model>>> {
    let items = Urls::find().all(db.conn()).await.map_err(DbError)?;
    Ok(Json(items))
}

#[public]
#[get("/:short_code", group = "/api/v1/shorten")]
#[errors(UrlsError)]
pub async fn redirect_url(db: Db, short_code: Path<String>) -> Result<Json<Model>> {
    let code = short_code.into_inner();
    let item = Urls::find()
        .filter(crate::entity::urls::Column::ShortCode.eq(&code))
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::not_found(format!("Code {} not found", code)))?;
    Ok(Json(item))
}
#[public]
#[post("/", group = "/api/v1/shorten")]
#[errors(UrlsError)]
pub async fn create_urls(db: Db, body: Json<CreateUrls>) -> Result<Json<Model>> {
    let input = body.into_inner();
    let item = ActiveModel {
        short_code: Set(String::new()),
        long_url: Set(input.long_url),
        created_at: Set(input.created_at),
        expires_at: Set(input.expires_at),
        click_count: Set(input.click_count),
        ..Default::default()
    };
    let inserted = item.insert(db.conn()).await.map_err(DbError)?;

    let mut active: ActiveModel = inserted.into_active_model();
    active.short_code = Set(base62::encode(active.id.clone().unwrap() as u128 + 6767u128));
    let result = active.update(db.conn()).await.map_err(DbError)?;
    Ok(Json(result))
}

#[put("/urlss/:id")]
#[errors(UrlsError)]
pub async fn update_urls(db: Db, id: Path<i32>, body: Json<UpdateUrls>) -> Result<Json<Model>> {
    let id = id.into_inner();
    let item = Urls::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::not_found(format!("Urls {} not found", id)))?;

    let update = body.into_inner();
    let mut active: ActiveModel = item.into_active_model();
    if let Some(val) = update.short_code {
        active.short_code = Set(val);
    }
    if let Some(val) = update.long_url {
        active.long_url = Set(val);
    }
    if let Some(val) = update.created_at {
        active.created_at = Set(val);
    }
    if let Some(val) = update.expires_at {
        active.expires_at = Set(val);
    }
    if let Some(val) = update.click_count {
        active.click_count = Set(val);
    }

    let result = active.update(db.conn()).await.map_err(DbError)?;
    Ok(Json(result))
}

#[delete("/urlss/:id")]
#[errors(UrlsError)]
pub async fn delete_urls(db: Db, id: Path<i32>) -> Result<Json<serde_json::Value>> {
    let id = id.into_inner();
    let result = Urls::delete_by_id(id)
        .exec(db.conn())
        .await
        .map_err(DbError)?;
    if result.rows_affected == 0 {
        return Err(Error::not_found(format!("Urls {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "deleted": id })))
}
