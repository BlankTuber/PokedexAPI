use crate::error::ApiResult;
use crate::models::encounter_method::{CreateEncounterMethod, EncounterMethod, UpdateEncounterMethod};
use crate::services::encounter_method_service::EncounterMethodService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/methods", format = "json", data = "<data>")]
pub async fn create_encounter_method(
    pool: &State<PgPool>,
    data: Json<CreateEncounterMethod>,
) -> ApiResult<Json<EncounterMethod>> {
    let result = EncounterMethodService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/methods/<id>")]
pub async fn get_encounter_method(pool: &State<PgPool>, id: i32) -> ApiResult<Json<EncounterMethod>> {
    let result = EncounterMethodService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/methods")]
pub async fn list_encounter_methods(pool: &State<PgPool>) -> ApiResult<Json<Vec<EncounterMethod>>> {
    let results = EncounterMethodService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[patch("/methods/<id>", format = "json", data = "<data>")]
pub async fn update_encounter_method(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateEncounterMethod>,
) -> ApiResult<Json<EncounterMethod>> {
    EncounterMethodService::update(pool.inner(), id, data.into_inner()).await?;
    let result = EncounterMethodService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/methods/<id>")]
pub async fn delete_encounter_method(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    EncounterMethodService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn encounter_method_routes() -> Vec<Route> {
    routes![
        create_encounter_method,
        get_encounter_method,
        list_encounter_methods,
        update_encounter_method,
        delete_encounter_method
    ]
}
