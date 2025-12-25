use crate::error::ApiResult;
use crate::models::type_matchup::{CreateTypeMatchup, TypeMatchup, UpdateTypeMatchup};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct TypeMatchupRepository;

impl TypeMatchupRepository {
    pub async fn create(pool: &PgPool, data: CreateTypeMatchup) -> ApiResult<TypeMatchup> {
        let result = sqlx::query_as!(
            TypeMatchup,
            r#"
            INSERT INTO type_matchups (attacking_type_id, defending_type_id, damage_factor)
            VALUES ($1, $2, $3)
            RETURNING attacking_type_id, defending_type_id, damage_factor
            "#,
            data.attacking_type_id,
            data.defending_type_id,
            data.damage_factor
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_key(pool: &PgPool, attacking_type_id: i32, defending_type_id: i32) -> ApiResult<TypeMatchup> {
        let result = sqlx::query_as!(
            TypeMatchup,
            r#"
            SELECT attacking_type_id, defending_type_id, damage_factor
            FROM type_matchups
            WHERE attacking_type_id = $1 AND defending_type_id = $2
            "#,
            attacking_type_id,
            defending_type_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<TypeMatchup>> {
        let results = sqlx::query_as!(
            TypeMatchup,
            r#"
            SELECT attacking_type_id, defending_type_id, damage_factor
            FROM type_matchups
            ORDER BY attacking_type_id, defending_type_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_attacking_type(pool: &PgPool, attacking_type_id: i32) -> ApiResult<Vec<TypeMatchup>> {
        let results = sqlx::query_as!(
            TypeMatchup,
            r#"
            SELECT attacking_type_id, defending_type_id, damage_factor
            FROM type_matchups
            WHERE attacking_type_id = $1
            ORDER BY defending_type_id
            "#,
            attacking_type_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_defending_type(pool: &PgPool, defending_type_id: i32) -> ApiResult<Vec<TypeMatchup>> {
        let results = sqlx::query_as!(
            TypeMatchup,
            r#"
            SELECT attacking_type_id, defending_type_id, damage_factor
            FROM type_matchups
            WHERE defending_type_id = $1
            ORDER BY attacking_type_id
            "#,
            defending_type_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, attacking_type_id: i32, defending_type_id: i32, data: UpdateTypeMatchup) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE type_matchups
            SET damage_factor = COALESCE($1, damage_factor)
            WHERE attacking_type_id = $2 AND defending_type_id = $3
            "#,
            data.damage_factor,
            attacking_type_id,
            defending_type_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Type matchup")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, attacking_type_id: i32, defending_type_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM type_matchups
            WHERE attacking_type_id = $1 AND defending_type_id = $2
            "#,
            attacking_type_id,
            defending_type_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Type matchup")?;

        Ok(())
    }
}
