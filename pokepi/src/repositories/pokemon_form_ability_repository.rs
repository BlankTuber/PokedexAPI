use crate::error::ApiResult;
use crate::models::pokemon_form_ability::{CreatePokemonFormAbility, PokemonFormAbility, UpdatePokemonFormAbility};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct PokemonFormAbilityRepository;

impl PokemonFormAbilityRepository {
    pub async fn create(pool: &PgPool, data: CreatePokemonFormAbility) -> ApiResult<PokemonFormAbility> {
        let result = sqlx::query_as!(
            PokemonFormAbility,
            r#"
            INSERT INTO pokemon_form_abilities (pokemon_form_game_id, ability_id, slot, is_hidden)
            VALUES ($1, $2, $3, $4)
            RETURNING pokemon_form_game_id, ability_id, slot, is_hidden
            "#,
            data.pokemon_form_game_id,
            data.ability_id,
            data.slot,
            data.is_hidden
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_key(pool: &PgPool, pokemon_form_game_id: i32, ability_id: i32) -> ApiResult<PokemonFormAbility> {
        let result = sqlx::query_as!(
            PokemonFormAbility,
            r#"
            SELECT pokemon_form_game_id, ability_id, slot, is_hidden
            FROM pokemon_form_abilities
            WHERE pokemon_form_game_id = $1 AND ability_id = $2
            "#,
            pokemon_form_game_id,
            ability_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<PokemonFormAbility>> {
        let results = sqlx::query_as!(
            PokemonFormAbility,
            r#"
            SELECT pokemon_form_game_id, ability_id, slot, is_hidden
            FROM pokemon_form_abilities
            ORDER BY pokemon_form_game_id, slot
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_form_game(pool: &PgPool, pokemon_form_game_id: i32) -> ApiResult<Vec<PokemonFormAbility>> {
        let results = sqlx::query_as!(
            PokemonFormAbility,
            r#"
            SELECT pokemon_form_game_id, ability_id, slot, is_hidden
            FROM pokemon_form_abilities
            WHERE pokemon_form_game_id = $1
            ORDER BY slot
            "#,
            pokemon_form_game_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, pokemon_form_game_id: i32, ability_id: i32, data: UpdatePokemonFormAbility) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE pokemon_form_abilities
            SET slot = COALESCE($1, slot),
                is_hidden = COALESCE($2, is_hidden)
            WHERE pokemon_form_game_id = $3 AND ability_id = $4
            "#,
            data.slot,
            data.is_hidden,
            pokemon_form_game_id,
            ability_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form ability")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, pokemon_form_game_id: i32, ability_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokemon_form_abilities
            WHERE pokemon_form_game_id = $1 AND ability_id = $2
            "#,
            pokemon_form_game_id,
            ability_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form ability")?;

        Ok(())
    }
}
