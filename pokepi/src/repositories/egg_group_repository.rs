use crate::error::ApiResult;
use crate::models::egg_group::{CreateEggGroup, EggGroup, UpdateEggGroup};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct EggGroupRepository;

impl EggGroupRepository {
    pub async fn create(pool: &PgPool, data: CreateEggGroup) -> ApiResult<EggGroup> {
        let result = sqlx::query_as!(
            EggGroup,
            r#"
            INSERT INTO egg_groups (egg_group_name, egg_group_identifier)
            VALUES ($1, $2)
            RETURNING egg_group_id, egg_group_name, egg_group_identifier
            "#,
            data.egg_group_name,
            data.egg_group_identifier
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<EggGroup> {
        let egg_group = sqlx::query_as!(
            EggGroup,
            r#"
            SELECT egg_group_id, egg_group_name, egg_group_identifier
            FROM egg_groups
            WHERE egg_group_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(egg_group)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<EggGroup>> {
        let egg_groups = sqlx::query_as!(
            EggGroup,
            r#"
            SELECT egg_group_id, egg_group_name, egg_group_identifier
            FROM egg_groups
            ORDER BY egg_group_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(egg_groups)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEggGroup) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE egg_groups
            SET egg_group_name = COALESCE($1, egg_group_name), 
                egg_group_identifier = COALESCE($2, egg_group_identifier)
            WHERE egg_group_id = $3
            "#,
            data.egg_group_name,
            data.egg_group_identifier,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Egg group")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM egg_groups WHERE egg_group_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Egg group")?;

        Ok(())
    }
}
