use crate::error::ApiResult;
use crate::models::encounter_condition_values::{CreateEncounterConditionValue, EncounterConditionValue, UpdateEncounterConditionValue};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct EncounterConditionValueRepository;

impl EncounterConditionValueRepository {
    pub async fn create(pool: &PgPool, data: CreateEncounterConditionValue) -> ApiResult<EncounterConditionValue> {
        let result = sqlx::query_as!(
            EncounterConditionValue,
            r#"
            INSERT INTO encounter_condition_values (value_name, value_identifier)
            VALUES ($1, $2)
            RETURNING encounter_condition_value_id, value_name, value_identifier
            "#,
            data.value_name,
            data.value_identifier
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<EncounterConditionValue> {
        let result = sqlx::query_as!(
            EncounterConditionValue,
            r#"
            SELECT encounter_condition_value_id, value_name, value_identifier
            FROM encounter_condition_values
            WHERE encounter_condition_value_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<EncounterConditionValue>> {
        let results = sqlx::query_as!(
            EncounterConditionValue,
            r#"
            SELECT encounter_condition_value_id, value_name, value_identifier
            FROM encounter_condition_values
            ORDER BY encounter_condition_value_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEncounterConditionValue) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE encounter_condition_values
            SET value_name = COALESCE($1, value_name),
                value_identifier = COALESCE($2, value_identifier)
            WHERE encounter_condition_value_id = $3
            "#,
            data.value_name,
            data.value_identifier,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter condition value")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM encounter_condition_values WHERE encounter_condition_value_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter condition value")?;

        Ok(())
    }
}
