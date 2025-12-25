use crate::error::ApiResult;
use crate::models::encounter_method::{CreateEncounterMethod, EncounterMethod, UpdateEncounterMethod};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct EncounterMethodRepository;

impl EncounterMethodRepository {
    pub async fn create(pool: &PgPool, data: CreateEncounterMethod) -> ApiResult<EncounterMethod> {
        let result = sqlx::query_as!(
            EncounterMethod,
            r#"
            INSERT INTO encounter_methods (method_name, method_identifier)
            VALUES ($1, $2)
            RETURNING encounter_method_id, method_name, method_identifier
            "#,
            data.method_name,
            data.method_identifier
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<EncounterMethod> {
        let result = sqlx::query_as!(
            EncounterMethod,
            r#"
            SELECT encounter_method_id, method_name, method_identifier
            FROM encounter_methods
            WHERE encounter_method_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<EncounterMethod>> {
        let results = sqlx::query_as!(
            EncounterMethod,
            r#"
            SELECT encounter_method_id, method_name, method_identifier
            FROM encounter_methods
            ORDER BY encounter_method_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEncounterMethod) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE encounter_methods
            SET method_name = COALESCE($1, method_name),
                method_identifier = COALESCE($2, method_identifier)
            WHERE encounter_method_id = $3
            "#,
            data.method_name,
            data.method_identifier,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter method")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM encounter_methods WHERE encounter_method_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter method")?;

        Ok(())
    }
}
