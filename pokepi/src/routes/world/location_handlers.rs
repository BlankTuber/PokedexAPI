use crate::error::ApiResult;
use crate::models::location::{CreateLocation, Location, UpdateLocation};
use crate::services::location_service::LocationService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/locations", format = "json", data = "<data>")]
pub async fn create_location(
    pool: &State<PgPool>,
    data: Json<CreateLocation>,
) -> ApiResult<Json<Location>> {
    let result = LocationService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/locations/<id>")]
pub async fn get_location(pool: &State<PgPool>, id: i32) -> ApiResult<Json<Location>> {
    let result = LocationService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/locations")]
pub async fn list_locations(pool: &State<PgPool>) -> ApiResult<Json<Vec<Location>>> {
    let results = LocationService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/regions/<region_id>/locations")]
pub async fn list_locations_by_region(
    pool: &State<PgPool>,
    region_id: i32,
) -> ApiResult<Json<Vec<Location>>> {
    let results = LocationService::list_by_region(pool.inner(), region_id).await?;
    Ok(Json(results))
}

#[patch("/locations/<id>", format = "json", data = "<data>")]
pub async fn update_location(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateLocation>,
) -> ApiResult<Json<Location>> {
    LocationService::update(pool.inner(), id, data.into_inner()).await?;
    let result = LocationService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/locations/<id>")]
pub async fn delete_location(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    LocationService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn location_routes() -> Vec<Route> {
    routes![
        create_location,
        get_location,
        list_locations,
        list_locations_by_region,
        update_location,
        delete_location
    ]
}
