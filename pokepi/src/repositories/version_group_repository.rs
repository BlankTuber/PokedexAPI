use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::version_group::{CreateVersionGroup, UpdateVersionGroup, VersionGroup},
    validators::common::CommonValidator,
};

pub struct VersionGroupRepository;

impl VersionGroupRepository {
    pub async fn create(pool: &PgPool, data: CreateVersionGroup) -> ApiResult<VersionGroup> {
        let result = sqlx::query_as!(
            VersionGroup,
            r#"
              INSERT INTO version_groups (version_group_name, version_group_identifier, generation)
              VALUES ($1, $2, $3)
              RETURNING version_group_id, version_group_name, version_group_identifier, generation
            "#,
            data.version_group_name,
            data.version_group_identifier,
            data.generation
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<VersionGroup> {
        let version_group = sqlx::query_as!(
            VersionGroup,
            r#"
              SELECT version_group_id, version_group_name, version_group_identifier, generation
              FROM version_groups
              WHERE version_group_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(version_group)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<VersionGroup>> {
        let version_groups = sqlx::query_as!(
            VersionGroup,
            r#"
              SELECT version_group_id, version_group_name, version_group_identifier, generation
              FROM version_groups
              ORDER BY version_group_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(version_groups)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateVersionGroup) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              UPDATE version_groups
              SET version_group_name = COALESCE($1, version_group_name),
                  version_group_identifier = COALESCE($2, version_group_identifier),
                  generation = COALESCE($3, generation)
              WHERE version_group_id = $4
            "#,
            data.version_group_name,
            data.version_group_identifier,
            data.generation,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Version group")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              DELETE FROM version_groups WHERE version_group_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Version group")?;

        Ok(())
    }
}
