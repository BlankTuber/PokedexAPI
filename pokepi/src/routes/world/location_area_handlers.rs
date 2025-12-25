use crate::error::ApiResult;
use crate::models::location_area::{CreateLocationArea, LocationArea, UpdateLocationArea};
use crate::services::location_area_service::LocationAreaService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/location-areas", format = "json", data = "<data>")]
pub async fn create_location_area(
    pool: &State<PgPool>,
    data: Json<CreateLocationArea>,
) -> ApiResult<Json<LocationArea>> {
    let result = LocationAreaService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/location-areas/<id>")]
pub async fn get_location_area(pool: &State<PgPool>, id: i32) -> ApiResult<Json<LocationArea>> {
    let result = LocationAreaService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/location-areas")]
pub async fn list_location_areas(pool: &State<PgPool>) -> ApiResult<Json<Vec<LocationArea>>> {
    let results = LocationAreaService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/locations/<location_id>/areas")]
pub async fn list_areas_by_location(
    pool: &State<PgPool>,
    location_id: i32,
) -> ApiResult<Json<Vec<LocationArea>>> {
    let results = LocationAreaService::list_by_location(pool.inner(), location_id).await?;
    Ok(Json(results))
}

#[patch("/location-areas/<id>", format = "json", data = "<data>")]
pub async fn update_location_area(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateLocationArea>,
) -> ApiResult<Json<LocationArea>> {
    LocationAreaService::update(pool.inner(), id, data.into_inner()).await?;
    let result = LocationAreaService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/location-areas/<id>")]
pub async fn delete_location_area(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    LocationAreaService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn location_area_routes() -> Vec<Route> {
    routes![
        create_location_area,
        get_location_area,
        list_location_areas,
        list_areas_by_location,
        update_location_area,
        delete_location_area
    ]
}
