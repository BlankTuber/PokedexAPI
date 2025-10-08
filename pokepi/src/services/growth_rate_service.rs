use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, GrowthRate, UpdateGrowthRate},
    repositories::growth_rate_repository::GrowthRateRepository,
    validators::growth_rate_validator::GrowthRateValidator,
};

pub struct GrowthRateService;

impl GrowthRateService {
    pub async fn create_growth_rate(
        pool: &PgPool,
        data: CreateGrowthRate,
    ) -> ApiResult<GrowthRate> {
        GrowthRateValidator::validate_create(&data)?;
        GrowthRateRepository::create(pool, data).await
    }

    pub async fn get_growth_rate(pool: &PgPool, id: i32) -> ApiResult<GrowthRate> {
        GrowthRateRepository::find_by_id(pool, id).await
    }

    pub async fn list_growth_rates(pool: &PgPool) -> ApiResult<Vec<GrowthRate>> {
        GrowthRateRepository::find_all(pool).await
    }

    pub async fn update_growth_rate(
        pool: &PgPool,
        id: i32,
        data: UpdateGrowthRate,
    ) -> ApiResult<()> {
        GrowthRateValidator::validate_update(&data)?;
        GrowthRateRepository::update(pool, id, data).await
    }

    pub async fn delete_growth_rate(pool: &PgPool, id: i32) -> ApiResult<()> {
        GrowthRateRepository::delete(pool, id).await
    }
}
