use crate::error::ApiResult;
use crate::models::location::{CreateLocation, Location, UpdateLocation};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct LocationRepository;

impl LocationRepository {
    pub async fn create(pool: &PgPool, data: CreateLocation) -> ApiResult<Location> {
        let result = sqlx::query_as!(
            Location,
            r#"
            INSERT INTO locations (location_name, location_identifier, region_id)
            VALUES ($1, $2, $3)
            RETURNING location_id, location_name, location_identifier, region_id
            "#,
            data.location_name,
            data.location_identifier,
            data.region_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<Location> {
        let result = sqlx::query_as!(
            Location,
            r#"
            SELECT location_id, location_name, location_identifier, region_id
            FROM locations
            WHERE location_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Location>> {
        let results = sqlx::query_as!(
            Location,
            r#"
            SELECT location_id, location_name, location_identifier, region_id
            FROM locations
            ORDER BY region_id, location_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_region(pool: &PgPool, region_id: i32) -> ApiResult<Vec<Location>> {
        let results = sqlx::query_as!(
            Location,
            r#"
            SELECT location_id, location_name, location_identifier, region_id
            FROM locations
            WHERE region_id = $1
            ORDER BY location_id
            "#,
            region_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateLocation) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE locations
            SET location_name = COALESCE($1, location_name),
                location_identifier = COALESCE($2, location_identifier)
            WHERE location_id = $3
            "#,
            data.location_name,
            data.location_identifier,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Location")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM locations WHERE location_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Location")?;

        Ok(())
    }
}
