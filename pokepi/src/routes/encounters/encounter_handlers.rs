use crate::error::ApiResult;
use crate::models::encounter::{CreateEncounter, Encounter, UpdateEncounter};
use crate::services::encounter_service::EncounterService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/encounters", format = "json", data = "<data>")]
pub async fn create_encounter(
    pool: &State<PgPool>,
    data: Json<CreateEncounter>,
) -> ApiResult<Json<Encounter>> {
    let result = EncounterService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/encounters/<id>")]
pub async fn get_encounter(pool: &State<PgPool>, id: i32) -> ApiResult<Json<Encounter>> {
    let result = EncounterService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/encounters")]
pub async fn list_encounters(pool: &State<PgPool>) -> ApiResult<Json<Vec<Encounter>>> {
    let results = EncounterService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/location-areas/<location_area_id>/encounters")]
pub async fn list_encounters_by_location(
    pool: &State<PgPool>,
    location_area_id: i32,
) -> ApiResult<Json<Vec<Encounter>>> {
    let results = EncounterService::list_by_location_area(pool.inner(), location_area_id).await?;
    Ok(Json(results))
}

#[get("/pokemon/<national_id>/encounters")]
pub async fn list_encounters_by_pokemon(
    pool: &State<PgPool>,
    national_id: i32,
) -> ApiResult<Json<Vec<Encounter>>> {
    let results = EncounterService::list_by_pokemon(pool.inner(), national_id).await?;
    Ok(Json(results))
}

#[patch("/encounters/<id>", format = "json", data = "<data>")]
pub async fn update_encounter(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateEncounter>,
) -> ApiResult<Json<Encounter>> {
    EncounterService::update(pool.inner(), id, data.into_inner()).await?;
    let result = EncounterService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/encounters/<id>")]
pub async fn delete_encounter(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    EncounterService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn encounter_routes() -> Vec<Route> {
    routes![
        create_encounter,
        get_encounter,
        list_encounters,
        list_encounters_by_location,
        list_encounters_by_pokemon,
        update_encounter,
        delete_encounter
    ]
}
