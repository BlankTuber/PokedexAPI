use sqlx::PgPool;

use crate::error::ApiResult;
use crate::models::pokedex_entry::{CreatePokedexEntry, PokedexEntry, UpdatePokedexEntry};

pub struct PokedexEntryRepository;

impl PokedexEntryRepository {
    pub async fn create(pool: &PgPool, data: CreatePokedexEntry) -> ApiResult<PokedexEntry> {
        let result = sqlx::query_as!(
            PokedexEntry,
            r#"
            INSERT INTO pokedex_entries (national_id, form_id, game_id, pokedex_number, entry_text)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING national_id, form_id, game_id, pokedex_number, entry_text
            "#,
            data.national_id,
            data.form_id,
            data.game_id,
            data.pokedex_number,
            data.entry_text
        )
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn get(
        pool: &PgPool,
        national_id: i32,
        form_id: Option<i32>,
        game_id: i32,
    ) -> ApiResult<PokedexEntry> {
        let result = if let Some(fid) = form_id {
            sqlx::query_as!(
                PokedexEntry,
                r#"
                SELECT national_id, form_id, game_id, pokedex_number, entry_text
                FROM pokedex_entries
                WHERE national_id = $1 AND form_id = $2 AND game_id = $3
                "#,
                national_id,
                fid,
                game_id
            )
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_as!(
                PokedexEntry,
                r#"
                SELECT national_id, form_id, game_id, pokedex_number, entry_text
                FROM pokedex_entries
                WHERE national_id = $1 AND form_id IS NULL AND game_id = $2
                "#,
                national_id,
                game_id
            )
            .fetch_one(pool)
            .await?
        };
        Ok(result)
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokedexEntry>> {
        let results = sqlx::query_as!(
            PokedexEntry,
            r#"
            SELECT national_id, form_id, game_id, pokedex_number, entry_text
            FROM pokedex_entries
            ORDER BY national_id, form_id NULLS FIRST, game_id
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(results)
    }

    pub async fn list_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokedexEntry>> {
        let results = sqlx::query_as!(
            PokedexEntry,
            r#"
            SELECT national_id, form_id, game_id, pokedex_number, entry_text
            FROM pokedex_entries
            WHERE national_id = $1
            ORDER BY form_id NULLS FIRST, game_id
            "#,
            national_id
        )
        .fetch_all(pool)
        .await?;
        Ok(results)
    }

    pub async fn list_by_game(pool: &PgPool, game_id: i32) -> ApiResult<Vec<PokedexEntry>> {
        let results = sqlx::query_as!(
            PokedexEntry,
            r#"
            SELECT national_id, form_id, game_id, pokedex_number, entry_text
            FROM pokedex_entries
            WHERE game_id = $1
            ORDER BY pokedex_number
            "#,
            game_id
        )
        .fetch_all(pool)
        .await?;
        Ok(results)
    }

    pub async fn update(
        pool: &PgPool,
        national_id: i32,
        form_id: Option<i32>,
        game_id: i32,
        data: UpdatePokedexEntry,
    ) -> ApiResult<u64> {
        let result = if let Some(fid) = form_id {
            sqlx::query!(
                r#"
                UPDATE pokedex_entries
                SET pokedex_number = COALESCE($1, pokedex_number),
                    entry_text = COALESCE($2, entry_text)
                WHERE national_id = $3 AND form_id = $4 AND game_id = $5
                "#,
                data.pokedex_number,
                data.entry_text,
                national_id,
                fid,
                game_id
            )
            .execute(pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                UPDATE pokedex_entries
                SET pokedex_number = COALESCE($1, pokedex_number),
                    entry_text = COALESCE($2, entry_text)
                WHERE national_id = $3 AND form_id IS NULL AND game_id = $4
                "#,
                data.pokedex_number,
                data.entry_text,
                national_id,
                game_id
            )
            .execute(pool)
            .await?
        };
        Ok(result.rows_affected())
    }

    pub async fn delete(
        pool: &PgPool,
        national_id: i32,
        form_id: Option<i32>,
        game_id: i32,
    ) -> ApiResult<u64> {
        let result = if let Some(fid) = form_id {
            sqlx::query!(
                r#"
                DELETE FROM pokedex_entries
                WHERE national_id = $1 AND form_id = $2 AND game_id = $3
                "#,
                national_id,
                fid,
                game_id
            )
            .execute(pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                DELETE FROM pokedex_entries
                WHERE national_id = $1 AND form_id IS NULL AND game_id = $2
                "#,
                national_id,
                game_id
            )
            .execute(pool)
            .await?
        };
        Ok(result.rows_affected())
    }
}
