use crate::error::ApiResult;
use crate::models::encounter_condition_values::{CreateEncounterConditionValue, EncounterConditionValue, UpdateEncounterConditionValue};
use crate::services::encounter_condition_value_service::EncounterConditionValueService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/condition-values", format = "json", data = "<data>")]
pub async fn create_condition_value(
    pool: &State<PgPool>,
    data: Json<CreateEncounterConditionValue>,
) -> ApiResult<Json<EncounterConditionValue>> {
    let result = EncounterConditionValueService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/condition-values/<id>")]
pub async fn get_condition_value(pool: &State<PgPool>, id: i32) -> ApiResult<Json<EncounterConditionValue>> {
    let result = EncounterConditionValueService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/condition-values")]
pub async fn list_condition_values(pool: &State<PgPool>) -> ApiResult<Json<Vec<EncounterConditionValue>>> {
    let results = EncounterConditionValueService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[patch("/condition-values/<id>", format = "json", data = "<data>")]
pub async fn update_condition_value(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateEncounterConditionValue>,
) -> ApiResult<Json<EncounterConditionValue>> {
    EncounterConditionValueService::update(pool.inner(), id, data.into_inner()).await?;
    let result = EncounterConditionValueService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/condition-values/<id>")]
pub async fn delete_condition_value(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    EncounterConditionValueService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn encounter_condition_value_routes() -> Vec<Route> {
    routes![
        create_condition_value,
        get_condition_value,
        list_condition_values,
        update_condition_value,
        delete_condition_value
    ]
}
