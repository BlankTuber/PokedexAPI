use crate::error::ApiResult;
use crate::models::type_matchup::{CreateTypeMatchup, TypeMatchup, UpdateTypeMatchup};
use crate::services::type_matchup_service::TypeMatchupService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/type-matchups", format = "json", data = "<data>")]
pub async fn create_type_matchup(
    pool: &State<PgPool>,
    data: Json<CreateTypeMatchup>,
) -> ApiResult<Json<TypeMatchup>> {
    let result = TypeMatchupService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/types/<attacking_type_id>/matchups/<defending_type_id>")]
pub async fn get_type_matchup(
    pool: &State<PgPool>,
    attacking_type_id: i32,
    defending_type_id: i32,
) -> ApiResult<Json<TypeMatchup>> {
    let result = TypeMatchupService::get(pool.inner(), attacking_type_id, defending_type_id).await?;
    Ok(Json(result))
}

#[get("/type-matchups")]
pub async fn list_type_matchups(pool: &State<PgPool>) -> ApiResult<Json<Vec<TypeMatchup>>> {
    let results = TypeMatchupService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/types/<attacking_type_id>/attacking")]
pub async fn list_matchups_by_attacking_type(
    pool: &State<PgPool>,
    attacking_type_id: i32,
) -> ApiResult<Json<Vec<TypeMatchup>>> {
    let results = TypeMatchupService::list_by_attacking_type(pool.inner(), attacking_type_id).await?;
    Ok(Json(results))
}

#[get("/types/<defending_type_id>/defending")]
pub async fn list_matchups_by_defending_type(
    pool: &State<PgPool>,
    defending_type_id: i32,
) -> ApiResult<Json<Vec<TypeMatchup>>> {
    let results = TypeMatchupService::list_by_defending_type(pool.inner(), defending_type_id).await?;
    Ok(Json(results))
}

#[patch("/types/<attacking_type_id>/matchups/<defending_type_id>", format = "json", data = "<data>")]
pub async fn update_type_matchup(
    pool: &State<PgPool>,
    attacking_type_id: i32,
    defending_type_id: i32,
    data: Json<UpdateTypeMatchup>,
) -> ApiResult<Json<TypeMatchup>> {
    TypeMatchupService::update(pool.inner(), attacking_type_id, defending_type_id, data.into_inner()).await?;
    let result = TypeMatchupService::get(pool.inner(), attacking_type_id, defending_type_id).await?;
    Ok(Json(result))
}

#[delete("/types/<attacking_type_id>/matchups/<defending_type_id>")]
pub async fn delete_type_matchup(
    pool: &State<PgPool>,
    attacking_type_id: i32,
    defending_type_id: i32,
) -> ApiResult<Status> {
    TypeMatchupService::delete(pool.inner(), attacking_type_id, defending_type_id).await?;
    Ok(Status::NoContent)
}

pub fn type_matchup_routes() -> Vec<Route> {
    routes![
        create_type_matchup,
        get_type_matchup,
        list_type_matchups,
        list_matchups_by_attacking_type,
        list_matchups_by_defending_type,
        update_type_matchup,
        delete_type_matchup
    ]
}
