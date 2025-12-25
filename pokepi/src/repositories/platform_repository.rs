use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::platform::{CreatePlatform, Platform, UpdatePlatform},
    validators::common::CommonValidator,
};

pub struct PlatformRepository;

impl PlatformRepository {
    pub async fn create(pool: &PgPool, data: CreatePlatform) -> ApiResult<Platform> {
        let result = sqlx::query_as!(
            Platform,
            r#"
              INSERT INTO platforms (platform_name, platform_identifier)
              VALUES ($1, $2)
              RETURNING platform_id, platform_name, platform_identifier
            "#,
            data.platform_name,
            data.platform_identifier
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<Platform> {
        let platform = sqlx::query_as!(
            Platform,
            r#"
              SELECT platform_id, platform_name, platform_identifier
              FROM platforms
              WHERE platform_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(platform)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Platform>> {
        let platforms = sqlx::query_as!(
            Platform,
            r#"
              SELECT platform_id, platform_name, platform_identifier
              FROM platforms
              ORDER BY platform_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(platforms)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdatePlatform) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              UPDATE platforms
              SET platform_name = COALESCE($1, platform_name),
                  platform_identifier = COALESCE($2, platform_identifier)
              WHERE platform_id = $3
            "#,
            data.platform_name,
            data.platform_identifier,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Platform")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              DELETE FROM platforms WHERE platform_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Platform")?;

        Ok(())
    }
}
