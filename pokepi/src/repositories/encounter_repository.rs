use crate::error::ApiResult;
use crate::models::encounter::{CreateEncounter, Encounter, UpdateEncounter};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct EncounterRepository;

impl EncounterRepository {
    pub async fn create(pool: &PgPool, data: CreateEncounter) -> ApiResult<Encounter> {
        let result = sqlx::query_as!(
            Encounter,
            r#"
            INSERT INTO encounters (national_id, form_id, game_id, location_area_id, encounter_method_id, chance)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING encounter_id, national_id, form_id, game_id, location_area_id, encounter_method_id, chance
            "#,
            data.national_id,
            data.form_id,
            data.game_id,
            data.location_area_id,
            data.encounter_method_id,
            data.chance
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<Encounter> {
        let result = sqlx::query_as!(
            Encounter,
            r#"
            SELECT encounter_id, national_id, form_id, game_id, location_area_id, encounter_method_id, chance
            FROM encounters
            WHERE encounter_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Encounter>> {
        let results = sqlx::query_as!(
            Encounter,
            r#"
            SELECT encounter_id, national_id, form_id, game_id, location_area_id, encounter_method_id, chance
            FROM encounters
            ORDER BY encounter_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_location_area(pool: &PgPool, location_area_id: i32) -> ApiResult<Vec<Encounter>> {
        let results = sqlx::query_as!(
            Encounter,
            r#"
            SELECT encounter_id, national_id, form_id, game_id, location_area_id, encounter_method_id, chance
            FROM encounters
            WHERE location_area_id = $1
            ORDER BY encounter_id
            "#,
            location_area_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<Encounter>> {
        let results = sqlx::query_as!(
            Encounter,
            r#"
            SELECT encounter_id, national_id, form_id, game_id, location_area_id, encounter_method_id, chance
            FROM encounters
            WHERE national_id = $1
            ORDER BY encounter_id
            "#,
            national_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEncounter) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE encounters
            SET national_id = COALESCE($1, national_id),
                form_id = COALESCE($2, form_id),
                game_id = COALESCE($3, game_id),
                location_area_id = COALESCE($4, location_area_id),
                encounter_method_id = COALESCE($5, encounter_method_id),
                chance = COALESCE($6, chance)
            WHERE encounter_id = $7
            "#,
            data.national_id,
            data.form_id,
            data.game_id,
            data.location_area_id,
            data.encounter_method_id,
            data.chance,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM encounters WHERE encounter_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter")?;

        Ok(())
    }
}
