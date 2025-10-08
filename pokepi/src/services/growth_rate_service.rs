use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, GrowthRate},
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
}
