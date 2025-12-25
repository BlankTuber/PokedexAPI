use crate::error::ApiResult;
use crate::models::pokemon_form::{CreatePokemonForm, PokemonForm, UpdatePokemonForm};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct PokemonFormRepository;

impl PokemonFormRepository {
    pub async fn create(pool: &PgPool, data: CreatePokemonForm) -> ApiResult<PokemonForm> {
        let result = sqlx::query_as!(
            PokemonForm,
            r#"
            INSERT INTO pokemon_forms (national_id, form_name, form_identifier, form_type, is_default, is_battle_only, sprite_name)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING form_id, national_id, form_name, form_identifier, form_type, is_default, is_battle_only, sprite_name
            "#,
            data.national_id,
            data.form_name,
            data.form_identifier,
            data.form_type,
            data.is_default,
            data.is_battle_only,
            data.sprite_name
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, form_id: i32) -> ApiResult<PokemonForm> {
        let form = sqlx::query_as!(
            PokemonForm,
            r#"
            SELECT form_id, national_id, form_name, form_identifier, form_type, is_default, is_battle_only, sprite_name
            FROM pokemon_forms
            WHERE form_id = $1
            "#,
            form_id
        )
        .fetch_one(pool)
        .await?;

        Ok(form)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<PokemonForm>> {
        let forms = sqlx::query_as!(
            PokemonForm,
            r#"
            SELECT form_id, national_id, form_name, form_identifier, form_type, is_default, is_battle_only, sprite_name
            FROM pokemon_forms
            ORDER BY national_id, form_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(forms)
    }

    pub async fn find_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokemonForm>> {
        let forms = sqlx::query_as!(
            PokemonForm,
            r#"
            SELECT form_id, national_id, form_name, form_identifier, form_type, is_default, is_battle_only, sprite_name
            FROM pokemon_forms
            WHERE national_id = $1
            ORDER BY form_id
            "#,
            national_id
        )
        .fetch_all(pool)
        .await?;

        Ok(forms)
    }

    pub async fn update(pool: &PgPool, form_id: i32, data: UpdatePokemonForm) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE pokemon_forms
            SET form_name = COALESCE($1, form_name),
                form_identifier = COALESCE($2, form_identifier),
                form_type = COALESCE($3, form_type),
                is_default = COALESCE($4, is_default),
                is_battle_only = COALESCE($5, is_battle_only),
                sprite_name = COALESCE($6, sprite_name)
            WHERE form_id = $7
            "#,
            data.form_name,
            data.form_identifier,
            data.form_type,
            data.is_default,
            data.is_battle_only,
            data.sprite_name,
            form_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, form_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokemon_forms WHERE form_id = $1
            "#,
            form_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form")?;

        Ok(())
    }
}
