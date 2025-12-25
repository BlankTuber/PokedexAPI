use crate::error::ApiResult;
use crate::models::location_area::{CreateLocationArea, LocationArea, UpdateLocationArea};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct LocationAreaRepository;

impl LocationAreaRepository {
    pub async fn create(pool: &PgPool, data: CreateLocationArea) -> ApiResult<LocationArea> {
        let result = sqlx::query_as!(
            LocationArea,
            r#"
            INSERT INTO location_areas (location_id, area_name, area_identifier)
            VALUES ($1, $2, $3)
            RETURNING location_area_id, location_id, area_name, area_identifier
            "#,
            data.location_id,
            data.area_name,
            data.area_identifier
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<LocationArea> {
        let result = sqlx::query_as!(
            LocationArea,
            r#"
            SELECT location_area_id, location_id, area_name, area_identifier
            FROM location_areas
            WHERE location_area_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<LocationArea>> {
        let results = sqlx::query_as!(
            LocationArea,
            r#"
            SELECT location_area_id, location_id, area_name, area_identifier
            FROM location_areas
            ORDER BY location_id, location_area_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_location(pool: &PgPool, location_id: i32) -> ApiResult<Vec<LocationArea>> {
        let results = sqlx::query_as!(
            LocationArea,
            r#"
            SELECT location_area_id, location_id, area_name, area_identifier
            FROM location_areas
            WHERE location_id = $1
            ORDER BY location_area_id
            "#,
            location_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateLocationArea) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE location_areas
            SET area_name = COALESCE($1, area_name),
                area_identifier = COALESCE($2, area_identifier)
            WHERE location_area_id = $3
            "#,
            data.area_name,
            data.area_identifier,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Location area")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM location_areas WHERE location_area_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Location area")?;

        Ok(())
    }
}
