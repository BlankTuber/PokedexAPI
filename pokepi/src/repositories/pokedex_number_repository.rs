use sqlx::PgPool;

use crate::error::ApiResult;
use crate::models::pokedex_number::{CreatePokedexNumber, PokedexNumber, UpdatePokedexNumber};

pub struct PokedexNumberRepository;

impl PokedexNumberRepository {
    pub async fn create(pool: &PgPool, data: CreatePokedexNumber) -> ApiResult<PokedexNumber> {
        let result = sqlx::query_as!(
            PokedexNumber,
            r#"
            INSERT INTO pokedex_numbers (national_id, pokedex_id, pokedex_number)
            VALUES ($1, $2, $3)
            RETURNING national_id, pokedex_id, pokedex_number
            "#,
            data.national_id,
            data.pokedex_id,
            data.pokedex_number
        )
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn get(pool: &PgPool, national_id: i32, pokedex_id: i32) -> ApiResult<PokedexNumber> {
        let result = sqlx::query_as!(
            PokedexNumber,
            r#"
            SELECT national_id, pokedex_id, pokedex_number
            FROM pokedex_numbers
            WHERE national_id = $1 AND pokedex_id = $2
            "#,
            national_id,
            pokedex_id
        )
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokedexNumber>> {
        let results = sqlx::query_as!(
            PokedexNumber,
            r#"
            SELECT national_id, pokedex_id, pokedex_number
            FROM pokedex_numbers
            ORDER BY pokedex_id, pokedex_number
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(results)
    }

    pub async fn list_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokedexNumber>> {
        let results = sqlx::query_as!(
            PokedexNumber,
            r#"
            SELECT national_id, pokedex_id, pokedex_number
            FROM pokedex_numbers
            WHERE national_id = $1
            ORDER BY pokedex_id
            "#,
            national_id
        )
        .fetch_all(pool)
        .await?;
        Ok(results)
    }

    pub async fn list_by_pokedex(pool: &PgPool, pokedex_id: i32) -> ApiResult<Vec<PokedexNumber>> {
        let results = sqlx::query_as!(
            PokedexNumber,
            r#"
            SELECT national_id, pokedex_id, pokedex_number
            FROM pokedex_numbers
            WHERE pokedex_id = $1
            ORDER BY pokedex_number
            "#,
            pokedex_id
        )
        .fetch_all(pool)
        .await?;
        Ok(results)
    }

    pub async fn update(
        pool: &PgPool,
        national_id: i32,
        pokedex_id: i32,
        data: UpdatePokedexNumber,
    ) -> ApiResult<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE pokedex_numbers
            SET pokedex_number = COALESCE($1, pokedex_number)
            WHERE national_id = $2 AND pokedex_id = $3
            "#,
            data.pokedex_number,
            national_id,
            pokedex_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &PgPool, national_id: i32, pokedex_id: i32) -> ApiResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokedex_numbers
            WHERE national_id = $1 AND pokedex_id = $2
            "#,
            national_id,
            pokedex_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }
}
