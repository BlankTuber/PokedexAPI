use crate::error::ApiResult;
use crate::models::ability_description::{AbilityDescription, CreateAbilityDescription, UpdateAbilityDescription};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct AbilityDescriptionRepository;

impl AbilityDescriptionRepository {
    pub async fn create(pool: &PgPool, data: CreateAbilityDescription) -> ApiResult<AbilityDescription> {
        let result = sqlx::query_as!(
            AbilityDescription,
            r#"
            INSERT INTO ability_descriptions (ability_id, version_group_id, flavor_text, short_effect, effect)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING ability_id, version_group_id, flavor_text, short_effect, effect
            "#,
            data.ability_id,
            data.version_group_id,
            data.flavor_text,
            data.short_effect,
            data.effect
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_key(pool: &PgPool, ability_id: i32, version_group_id: i32) -> ApiResult<AbilityDescription> {
        let result = sqlx::query_as!(
            AbilityDescription,
            r#"
            SELECT ability_id, version_group_id, flavor_text, short_effect, effect
            FROM ability_descriptions
            WHERE ability_id = $1 AND version_group_id = $2
            "#,
            ability_id,
            version_group_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<AbilityDescription>> {
        let results = sqlx::query_as!(
            AbilityDescription,
            r#"
            SELECT ability_id, version_group_id, flavor_text, short_effect, effect
            FROM ability_descriptions
            ORDER BY ability_id, version_group_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_ability(pool: &PgPool, ability_id: i32) -> ApiResult<Vec<AbilityDescription>> {
        let results = sqlx::query_as!(
            AbilityDescription,
            r#"
            SELECT ability_id, version_group_id, flavor_text, short_effect, effect
            FROM ability_descriptions
            WHERE ability_id = $1
            ORDER BY version_group_id
            "#,
            ability_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, ability_id: i32, version_group_id: i32, data: UpdateAbilityDescription) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE ability_descriptions
            SET flavor_text = COALESCE($1, flavor_text),
                short_effect = COALESCE($2, short_effect),
                effect = COALESCE($3, effect)
            WHERE ability_id = $4 AND version_group_id = $5
            "#,
            data.flavor_text,
            data.short_effect,
            data.effect,
            ability_id,
            version_group_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Ability description")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, ability_id: i32, version_group_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM ability_descriptions
            WHERE ability_id = $1 AND version_group_id = $2
            "#,
            ability_id,
            version_group_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Ability description")?;

        Ok(())
    }
}
