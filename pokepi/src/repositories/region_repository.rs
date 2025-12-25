use crate::error::ApiResult;
use crate::models::region::{CreateRegion, Region, UpdateRegion};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct RegionRepository;

impl RegionRepository {
    pub async fn create(pool: &PgPool, data: CreateRegion) -> ApiResult<Region> {
        let result = sqlx::query_as!(
            Region,
            r#"
            INSERT INTO regions (region_name, region_identifier, generation_introduced)
            VALUES ($1, $2, $3)
            RETURNING region_id, region_name, region_identifier, generation_introduced
            "#,
            data.region_name,
            data.region_identifier,
            data.generation_introduced
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<Region> {
        let result = sqlx::query_as!(
            Region,
            r#"
            SELECT region_id, region_name, region_identifier, generation_introduced
            FROM regions
            WHERE region_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Region>> {
        let results = sqlx::query_as!(
            Region,
            r#"
            SELECT region_id, region_name, region_identifier, generation_introduced
            FROM regions
            ORDER BY region_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateRegion) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE regions
            SET region_name = COALESCE($1, region_name),
                region_identifier = COALESCE($2, region_identifier),
                generation_introduced = COALESCE($3, generation_introduced)
            WHERE region_id = $4
            "#,
            data.region_name,
            data.region_identifier,
            data.generation_introduced,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Region")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM regions WHERE region_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Region")?;

        Ok(())
    }
}
