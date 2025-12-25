use rocket::{Route, State, http::Status, serde::json::Json};
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::platform::{CreatePlatform, Platform, UpdatePlatform},
    services::platform_service::PlatformService,
};

#[post("/platforms", format = "json", data = "<data>")]
pub async fn create_platform(
    pool: &State<PgPool>,
    data: Json<CreatePlatform>,
) -> ApiResult<Json<Platform>> {
    let platform = PlatformService::create_platform(pool.inner(), data.into_inner()).await?;
    Ok(Json(platform))
}

#[get("/platforms/<id>")]
pub async fn get_platform(pool: &State<PgPool>, id: i32) -> ApiResult<Json<Platform>> {
    let platform = PlatformService::get_platform(pool.inner(), id).await?;
    Ok(Json(platform))
}

#[get("/platforms")]
pub async fn list_platforms(pool: &State<PgPool>) -> ApiResult<Json<Vec<Platform>>> {
    let platforms = PlatformService::list_platforms(pool.inner()).await?;
    Ok(Json(platforms))
}

#[patch("/platforms/<id>", format = "json", data = "<data>")]
pub async fn update_platform(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdatePlatform>,
) -> ApiResult<Json<Platform>> {
    PlatformService::update_platform(pool.inner(), id, data.into_inner()).await?;
    let platform = PlatformService::get_platform(pool.inner(), id).await?;
    Ok(Json(platform))
}

#[delete("/platforms/<id>")]
pub async fn delete_platform(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    PlatformService::delete_platform(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn platform_routes() -> Vec<Route> {
    routes![
        create_platform,
        get_platform,
        list_platforms,
        update_platform,
        delete_platform
    ]
}
