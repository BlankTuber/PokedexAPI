use crate::error::ApiResult;
use crate::models::ability_description::{AbilityDescription, CreateAbilityDescription, UpdateAbilityDescription};
use crate::services::ability_description_service::AbilityDescriptionService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/ability-descriptions", format = "json", data = "<data>")]
pub async fn create_ability_description(
    pool: &State<PgPool>,
    data: Json<CreateAbilityDescription>,
) -> ApiResult<Json<AbilityDescription>> {
    let result = AbilityDescriptionService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/abilities/<ability_id>/descriptions/<version_group_id>")]
pub async fn get_ability_description(
    pool: &State<PgPool>,
    ability_id: i32,
    version_group_id: i32,
) -> ApiResult<Json<AbilityDescription>> {
    let result = AbilityDescriptionService::get(pool.inner(), ability_id, version_group_id).await?;
    Ok(Json(result))
}

#[get("/ability-descriptions")]
pub async fn list_ability_descriptions(pool: &State<PgPool>) -> ApiResult<Json<Vec<AbilityDescription>>> {
    let results = AbilityDescriptionService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/abilities/<ability_id>/descriptions")]
pub async fn list_descriptions_by_ability(
    pool: &State<PgPool>,
    ability_id: i32,
) -> ApiResult<Json<Vec<AbilityDescription>>> {
    let results = AbilityDescriptionService::list_by_ability(pool.inner(), ability_id).await?;
    Ok(Json(results))
}

#[patch("/abilities/<ability_id>/descriptions/<version_group_id>", format = "json", data = "<data>")]
pub async fn update_ability_description(
    pool: &State<PgPool>,
    ability_id: i32,
    version_group_id: i32,
    data: Json<UpdateAbilityDescription>,
) -> ApiResult<Json<AbilityDescription>> {
    AbilityDescriptionService::update(pool.inner(), ability_id, version_group_id, data.into_inner()).await?;
    let result = AbilityDescriptionService::get(pool.inner(), ability_id, version_group_id).await?;
    Ok(Json(result))
}

#[delete("/abilities/<ability_id>/descriptions/<version_group_id>")]
pub async fn delete_ability_description(
    pool: &State<PgPool>,
    ability_id: i32,
    version_group_id: i32,
) -> ApiResult<Status> {
    AbilityDescriptionService::delete(pool.inner(), ability_id, version_group_id).await?;
    Ok(Status::NoContent)
}

pub fn ability_description_routes() -> Vec<Route> {
    routes![
        create_ability_description,
        get_ability_description,
        list_ability_descriptions,
        list_descriptions_by_ability,
        update_ability_description,
        delete_ability_description
    ]
}
