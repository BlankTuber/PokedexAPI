use rocket::{Route, State, http::Status, serde::json::Json};
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::version_group::{CreateVersionGroup, UpdateVersionGroup, VersionGroup},
    services::version_group_service::VersionGroupService,
};

#[post("/version-groups", format = "json", data = "<data>")]
pub async fn create_version_group(
    pool: &State<PgPool>,
    data: Json<CreateVersionGroup>,
) -> ApiResult<Json<VersionGroup>> {
    let version_group =
        VersionGroupService::create_version_group(pool.inner(), data.into_inner()).await?;
    Ok(Json(version_group))
}

#[get("/version-groups/<id>")]
pub async fn get_version_group(pool: &State<PgPool>, id: i32) -> ApiResult<Json<VersionGroup>> {
    let version_group = VersionGroupService::get_version_group(pool.inner(), id).await?;
    Ok(Json(version_group))
}

#[get("/version-groups")]
pub async fn list_version_groups(pool: &State<PgPool>) -> ApiResult<Json<Vec<VersionGroup>>> {
    let version_groups = VersionGroupService::list_version_groups(pool.inner()).await?;
    Ok(Json(version_groups))
}

#[patch("/version-groups/<id>", format = "json", data = "<data>")]
pub async fn update_version_group(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateVersionGroup>,
) -> ApiResult<Json<VersionGroup>> {
    VersionGroupService::update_version_group(pool.inner(), id, data.into_inner()).await?;
    let version_group = VersionGroupService::get_version_group(pool.inner(), id).await?;
    Ok(Json(version_group))
}

#[delete("/version-groups/<id>")]
pub async fn delete_version_group(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    VersionGroupService::delete_version_group(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn version_group_routes() -> Vec<Route> {
    routes![
        create_version_group,
        get_version_group,
        list_version_groups,
        update_version_group,
        delete_version_group
    ]
}
