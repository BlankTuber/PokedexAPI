use rocket::{Route, State, http::Status, serde::json::Json};
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, GrowthRate, UpdateGrowthRate},
    services::growth_rate_service::GrowthRateService,
};

#[post("/growth-rates", format = "json", data = "<data>")]
pub async fn create_growth_rate(
    pool: &State<PgPool>,
    data: Json<CreateGrowthRate>,
) -> ApiResult<Json<GrowthRate>> {
    let growth_rate =
        GrowthRateService::create_growth_rate(pool.inner(), data.into_inner()).await?;
    Ok(Json(growth_rate))
}

#[get("/growth-rates/<id>")]
pub async fn get_growth_rate(pool: &State<PgPool>, id: i32) -> ApiResult<Json<GrowthRate>> {
    let growth_rate = GrowthRateService::get_growth_rate(pool.inner(), id).await?;
    Ok(Json(growth_rate))
}

#[get("/growth-rates")]
pub async fn list_growth_rates(pool: &State<PgPool>) -> ApiResult<Json<Vec<GrowthRate>>> {
    let growth_rates = GrowthRateService::list_growth_rates(pool.inner()).await?;
    Ok(Json(growth_rates))
}

#[patch("/growth-rates/<id>", format = "json", data = "<data>")]
pub async fn update_growth_rate(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateGrowthRate>,
) -> ApiResult<Json<GrowthRate>> {
    GrowthRateService::update_growth_rate(pool.inner(), id, data.into_inner()).await?;
    let growth_rate = GrowthRateService::get_growth_rate(pool.inner(), id).await?;
    Ok(Json(growth_rate))
}

#[delete("/growth-rates/<id>")]
pub async fn delete_growth_rate(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    GrowthRateService::delete_growth_rate(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn growth_rate_routes() -> Vec<Route> {
    routes![
        create_growth_rate,
        get_growth_rate,
        list_growth_rates,
        update_growth_rate,
        delete_growth_rate
    ]
}
