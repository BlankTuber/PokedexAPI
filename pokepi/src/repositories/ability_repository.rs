use crate::error::ApiResult;
use crate::models::ability::{Ability, CreateAbility, UpdateAbility};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct AbilityRepository;

impl AbilityRepository {
    pub async fn create(pool: &PgPool, data: CreateAbility) -> ApiResult<Ability> {
        let result = sqlx::query_as!(
            Ability,
            r#"
            INSERT INTO abilities (ability_name, ability_identifier, generation_introduced, is_main_series)
            VALUES ($1, $2, $3, $4)
            RETURNING ability_id, ability_name, ability_identifier, generation_introduced, is_main_series
            "#,
            data.ability_name,
            data.ability_identifier,
            data.generation_introduced,
            data.is_main_series
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<Ability> {
        let result = sqlx::query_as!(
            Ability,
            r#"
            SELECT ability_id, ability_name, ability_identifier, generation_introduced, is_main_series
            FROM abilities
            WHERE ability_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Ability>> {
        let results = sqlx::query_as!(
            Ability,
            r#"
            SELECT ability_id, ability_name, ability_identifier, generation_introduced, is_main_series
            FROM abilities
            ORDER BY ability_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateAbility) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE abilities
            SET ability_name = COALESCE($1, ability_name),
                ability_identifier = COALESCE($2, ability_identifier),
                generation_introduced = COALESCE($3, generation_introduced),
                is_main_series = COALESCE($4, is_main_series)
            WHERE ability_id = $5
            "#,
            data.ability_name,
            data.ability_identifier,
            data.generation_introduced,
            data.is_main_series,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Ability")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM abilities WHERE ability_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Ability")?;

        Ok(())
    }
}
