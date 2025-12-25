use crate::error::ApiResult;
use crate::models::pokemon_form_type::{CreatePokemonFormType, PokemonFormType, UpdatePokemonFormType};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct PokemonFormTypeRepository;

impl PokemonFormTypeRepository {
    pub async fn create(pool: &PgPool, data: CreatePokemonFormType) -> ApiResult<PokemonFormType> {
        let result = sqlx::query_as!(
            PokemonFormType,
            r#"
            INSERT INTO pokemon_form_types (pokemon_form_game_id, type_id, slot)
            VALUES ($1, $2, $3)
            RETURNING pokemon_form_game_id, type_id, slot
            "#,
            data.pokemon_form_game_id,
            data.type_id,
            data.slot
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_key(pool: &PgPool, pokemon_form_game_id: i32, type_id: i32) -> ApiResult<PokemonFormType> {
        let result = sqlx::query_as!(
            PokemonFormType,
            r#"
            SELECT pokemon_form_game_id, type_id, slot
            FROM pokemon_form_types
            WHERE pokemon_form_game_id = $1 AND type_id = $2
            "#,
            pokemon_form_game_id,
            type_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<PokemonFormType>> {
        let results = sqlx::query_as!(
            PokemonFormType,
            r#"
            SELECT pokemon_form_game_id, type_id, slot
            FROM pokemon_form_types
            ORDER BY pokemon_form_game_id, slot
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_form_game(pool: &PgPool, pokemon_form_game_id: i32) -> ApiResult<Vec<PokemonFormType>> {
        let results = sqlx::query_as!(
            PokemonFormType,
            r#"
            SELECT pokemon_form_game_id, type_id, slot
            FROM pokemon_form_types
            WHERE pokemon_form_game_id = $1
            ORDER BY slot
            "#,
            pokemon_form_game_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, pokemon_form_game_id: i32, type_id: i32, data: UpdatePokemonFormType) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE pokemon_form_types
            SET slot = COALESCE($1, slot)
            WHERE pokemon_form_game_id = $2 AND type_id = $3
            "#,
            data.slot,
            pokemon_form_game_id,
            type_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form type")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, pokemon_form_game_id: i32, type_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokemon_form_types
            WHERE pokemon_form_game_id = $1 AND type_id = $2
            "#,
            pokemon_form_game_id,
            type_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form type")?;

        Ok(())
    }
}
