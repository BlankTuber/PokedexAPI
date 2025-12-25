use crate::error::ApiResult;
use crate::models::region::{CreateRegion, Region, UpdateRegion};
use crate::services::region_service::RegionService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/regions", format = "json", data = "<data>")]
pub async fn create_region(
    pool: &State<PgPool>,
    data: Json<CreateRegion>,
) -> ApiResult<Json<Region>> {
    let result = RegionService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/regions/<id>")]
pub async fn get_region(pool: &State<PgPool>, id: i32) -> ApiResult<Json<Region>> {
    let result = RegionService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/regions")]
pub async fn list_regions(pool: &State<PgPool>) -> ApiResult<Json<Vec<Region>>> {
    let results = RegionService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[patch("/regions/<id>", format = "json", data = "<data>")]
pub async fn update_region(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateRegion>,
) -> ApiResult<Json<Region>> {
    RegionService::update(pool.inner(), id, data.into_inner()).await?;
    let result = RegionService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/regions/<id>")]
pub async fn delete_region(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    RegionService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn region_routes() -> Vec<Route> {
    routes![
        create_region,
        get_region,
        list_regions,
        update_region,
        delete_region
    ]
}
