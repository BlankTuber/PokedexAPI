use crate::error::ApiResult;
use crate::models::ability::{Ability, CreateAbility, UpdateAbility};
use crate::services::ability_service::AbilityService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/abilities", format = "json", data = "<data>")]
pub async fn create_ability(
    pool: &State<PgPool>,
    data: Json<CreateAbility>,
) -> ApiResult<Json<Ability>> {
    let result = AbilityService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/abilities/<id>")]
pub async fn get_ability(pool: &State<PgPool>, id: i32) -> ApiResult<Json<Ability>> {
    let result = AbilityService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/abilities")]
pub async fn list_abilities(pool: &State<PgPool>) -> ApiResult<Json<Vec<Ability>>> {
    let results = AbilityService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[patch("/abilities/<id>", format = "json", data = "<data>")]
pub async fn update_ability(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateAbility>,
) -> ApiResult<Json<Ability>> {
    AbilityService::update(pool.inner(), id, data.into_inner()).await?;
    let result = AbilityService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/abilities/<id>")]
pub async fn delete_ability(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    AbilityService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn ability_routes() -> Vec<Route> {
    routes![
        create_ability,
        get_ability,
        list_abilities,
        update_ability,
        delete_ability
    ]
}
