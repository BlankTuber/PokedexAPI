use crate::error::ApiResult;
use crate::models::pokemon_type::{CreatePokemonType, PokemonType, UpdatePokemonType};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct PokemonTypeRepository;

impl PokemonTypeRepository {
    pub async fn create(pool: &PgPool, data: CreatePokemonType) -> ApiResult<PokemonType> {
        let result = sqlx::query_as!(
            PokemonType,
            r#"
            INSERT INTO pokemon_types (type_name, type_identifier, generation_introduced)
            VALUES ($1, $2, $3)
            RETURNING type_id, type_name, type_identifier, generation_introduced
            "#,
            data.type_name,
            data.type_identifier,
            data.generation_introduced
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<PokemonType> {
        let result = sqlx::query_as!(
            PokemonType,
            r#"
            SELECT type_id, type_name, type_identifier, generation_introduced
            FROM pokemon_types
            WHERE type_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<PokemonType>> {
        let results = sqlx::query_as!(
            PokemonType,
            r#"
            SELECT type_id, type_name, type_identifier, generation_introduced
            FROM pokemon_types
            ORDER BY type_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdatePokemonType) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE pokemon_types
            SET type_name = COALESCE($1, type_name),
                type_identifier = COALESCE($2, type_identifier),
                generation_introduced = COALESCE($3, generation_introduced)
            WHERE type_id = $4
            "#,
            data.type_name,
            data.type_identifier,
            data.generation_introduced,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon type")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokemon_types WHERE type_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon type")?;

        Ok(())
    }
}
