use crate::error::ApiResult;
use crate::models::egg_group::{CreateEggGroup, EggGroup, UpdateEggGroup};
use crate::services::egg_group_service::EggGroupService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/egg-groups", format = "json", data = "<data>")]
pub async fn create_egg_group(
    pool: &State<PgPool>,
    data: Json<CreateEggGroup>,
) -> ApiResult<Json<EggGroup>> {
    let egg_group = EggGroupService::create_egg_group(pool.inner(), data.into_inner()).await?;
    Ok(Json(egg_group))
}

#[get("/egg-groups/<id>")]
pub async fn get_egg_group(pool: &State<PgPool>, id: i32) -> ApiResult<Json<EggGroup>> {
    let egg_group = EggGroupService::get_egg_group(pool.inner(), id).await?;
    Ok(Json(egg_group))
}

#[get("/egg-groups")]
pub async fn list_egg_groups(pool: &State<PgPool>) -> ApiResult<Json<Vec<EggGroup>>> {
    let egg_groups = EggGroupService::list_egg_groups(pool.inner()).await?;
    Ok(Json(egg_groups))
}

#[patch("/egg-groups/<id>", format = "json", data = "<data>")]
pub async fn update_egg_group(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateEggGroup>,
) -> ApiResult<Json<EggGroup>> {
    EggGroupService::update_egg_group(pool.inner(), id, data.into_inner()).await?;
    let egg_group = EggGroupService::get_egg_group(pool.inner(), id).await?;
    Ok(Json(egg_group))
}

#[delete("/egg-groups/<id>")]
pub async fn delete_egg_group(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    EggGroupService::delete_egg_group(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn egg_group_routes() -> Vec<Route> {
    routes![
        create_egg_group,
        get_egg_group,
        list_egg_groups,
        update_egg_group,
        delete_egg_group
    ]
}
