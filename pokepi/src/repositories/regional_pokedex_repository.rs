use crate::error::ApiResult;
use crate::models::regional_pokedex::{CreateRegionalPokedex, RegionalPokedex, UpdateRegionalPokedex};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct RegionalPokedexRepository;

impl RegionalPokedexRepository {
    pub async fn create(pool: &PgPool, data: CreateRegionalPokedex) -> ApiResult<RegionalPokedex> {
        let result = sqlx::query_as!(
            RegionalPokedex,
            r#"
            INSERT INTO regional_pokedexes (pokedex_name, pokedex_identifier, region_id, version_group_id, is_main_series)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING pokedex_id, pokedex_name, pokedex_identifier, region_id, version_group_id, is_main_series
            "#,
            data.pokedex_name,
            data.pokedex_identifier,
            data.region_id,
            data.version_group_id,
            data.is_main_series
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<RegionalPokedex> {
        let result = sqlx::query_as!(
            RegionalPokedex,
            r#"
            SELECT pokedex_id, pokedex_name, pokedex_identifier, region_id, version_group_id, is_main_series
            FROM regional_pokedexes
            WHERE pokedex_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<RegionalPokedex>> {
        let results = sqlx::query_as!(
            RegionalPokedex,
            r#"
            SELECT pokedex_id, pokedex_name, pokedex_identifier, region_id, version_group_id, is_main_series
            FROM regional_pokedexes
            ORDER BY pokedex_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_region(pool: &PgPool, region_id: i32) -> ApiResult<Vec<RegionalPokedex>> {
        let results = sqlx::query_as!(
            RegionalPokedex,
            r#"
            SELECT pokedex_id, pokedex_name, pokedex_identifier, region_id, version_group_id, is_main_series
            FROM regional_pokedexes
            WHERE region_id = $1
            ORDER BY pokedex_id
            "#,
            region_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateRegionalPokedex) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE regional_pokedexes
            SET pokedex_name = COALESCE($1, pokedex_name),
                pokedex_identifier = COALESCE($2, pokedex_identifier),
                is_main_series = COALESCE($3, is_main_series)
            WHERE pokedex_id = $4
            "#,
            data.pokedex_name,
            data.pokedex_identifier,
            data.is_main_series,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Regional pokedex")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM regional_pokedexes WHERE pokedex_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Regional pokedex")?;

        Ok(())
    }
}
