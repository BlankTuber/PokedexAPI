use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, GrowthRate},
};

pub struct GrowthRateRepository;

impl GrowthRateRepository {
    pub async fn create(pool: &PgPool, data: CreateGrowthRate) -> ApiResult<GrowthRate> {
        let result = sqlx::query_as!(
            GrowthRate,
            r#"
              INSERT INTO growth_rates (growth_rate_name, growth_rate_identifier, formula)
              VALUES ($1, $2, $3)
              RETURNING growth_rate_id, growth_rate_name, growth_rate_identifier, formula
            "#,
            data.growth_rate_name,
            data.growth_rate_identifier,
            data.formula
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
}
