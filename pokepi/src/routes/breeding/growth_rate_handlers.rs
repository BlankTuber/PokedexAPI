use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, GrowthRate},
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

pub fn growth_rate_routes() -> Vec<Route> {
    routes![create_growth_rate]
}
