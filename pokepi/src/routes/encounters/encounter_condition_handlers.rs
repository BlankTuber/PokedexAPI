use crate::error::ApiResult;
use crate::models::encounter_condition::{CreateEncounterCondition, EncounterCondition};
use crate::services::encounter_condition_service::EncounterConditionService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/conditions", format = "json", data = "<data>")]
pub async fn create_encounter_condition(
    pool: &State<PgPool>,
    data: Json<CreateEncounterCondition>,
) -> ApiResult<Json<EncounterCondition>> {
    let result = EncounterConditionService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/encounters/<encounter_id>/conditions/<condition_value_id>")]
pub async fn get_encounter_condition(
    pool: &State<PgPool>,
    encounter_id: i32,
    condition_value_id: i32,
) -> ApiResult<Json<EncounterCondition>> {
    let result = EncounterConditionService::get(pool.inner(), encounter_id, condition_value_id).await?;
    Ok(Json(result))
}

#[get("/conditions")]
pub async fn list_encounter_conditions(pool: &State<PgPool>) -> ApiResult<Json<Vec<EncounterCondition>>> {
    let results = EncounterConditionService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/encounters/<encounter_id>/conditions")]
pub async fn list_conditions_by_encounter(
    pool: &State<PgPool>,
    encounter_id: i32,
) -> ApiResult<Json<Vec<EncounterCondition>>> {
    let results = EncounterConditionService::list_by_encounter(pool.inner(), encounter_id).await?;
    Ok(Json(results))
}

#[delete("/encounters/<encounter_id>/conditions/<condition_value_id>")]
pub async fn delete_encounter_condition(
    pool: &State<PgPool>,
    encounter_id: i32,
    condition_value_id: i32,
) -> ApiResult<Status> {
    EncounterConditionService::delete(pool.inner(), encounter_id, condition_value_id).await?;
    Ok(Status::NoContent)
}

pub fn encounter_condition_routes() -> Vec<Route> {
    routes![
        create_encounter_condition,
        get_encounter_condition,
        list_encounter_conditions,
        list_conditions_by_encounter,
        delete_encounter_condition
    ]
}
