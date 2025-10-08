use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, GrowthRate, UpdateGrowthRate},
    validators::common::CommonValidator,
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

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<GrowthRate> {
        let growth_rate = sqlx::query_as!(
            GrowthRate,
            r#"
              SELECT growth_rate_id, growth_rate_name, growth_rate_identifier, formula
              FROM growth_rates
              WHERE growth_rate_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(growth_rate)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<GrowthRate>> {
        let growth_rates = sqlx::query_as!(
            GrowthRate,
            r#"
              SELECT growth_rate_id, growth_rate_name, growth_rate_identifier, formula
              FROM growth_rates
              ORDER BY growth_rate_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(growth_rates)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateGrowthRate) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              UPDATE growth_rates
              SET growth_rate_name = COALESCE($1, growth_rate_name),
                  growth_rate_identifier = COALESCE($2, growth_rate_identifier),
                  formula = COALESCE($3, formula)
              WHERE growth_rate_id = $4
            "#,
            data.growth_rate_name,
            data.growth_rate_identifier,
            data.formula,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Growth rate")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              DELETE FROM growth_rates WHERE growth_rate_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Growth rate")?;

        Ok(())
    }
}
