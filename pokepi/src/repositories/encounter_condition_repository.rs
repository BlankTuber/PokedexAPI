use crate::error::ApiResult;
use crate::models::encounter_condition::{CreateEncounterCondition, EncounterCondition};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct EncounterConditionRepository;

impl EncounterConditionRepository {
    pub async fn create(pool: &PgPool, data: CreateEncounterCondition) -> ApiResult<EncounterCondition> {
        let result = sqlx::query_as!(
            EncounterCondition,
            r#"
            INSERT INTO encounter_conditions (encounter_id, encounter_condition_value_id)
            VALUES ($1, $2)
            RETURNING encounter_id, encounter_condition_value_id
            "#,
            data.encounter_id,
            data.encounter_condition_value_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_key(pool: &PgPool, encounter_id: i32, condition_value_id: i32) -> ApiResult<EncounterCondition> {
        let result = sqlx::query_as!(
            EncounterCondition,
            r#"
            SELECT encounter_id, encounter_condition_value_id
            FROM encounter_conditions
            WHERE encounter_id = $1 AND encounter_condition_value_id = $2
            "#,
            encounter_id,
            condition_value_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<EncounterCondition>> {
        let results = sqlx::query_as!(
            EncounterCondition,
            r#"
            SELECT encounter_id, encounter_condition_value_id
            FROM encounter_conditions
            ORDER BY encounter_id, encounter_condition_value_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_encounter(pool: &PgPool, encounter_id: i32) -> ApiResult<Vec<EncounterCondition>> {
        let results = sqlx::query_as!(
            EncounterCondition,
            r#"
            SELECT encounter_id, encounter_condition_value_id
            FROM encounter_conditions
            WHERE encounter_id = $1
            ORDER BY encounter_condition_value_id
            "#,
            encounter_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn delete(pool: &PgPool, encounter_id: i32, condition_value_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM encounter_conditions
            WHERE encounter_id = $1 AND encounter_condition_value_id = $2
            "#,
            encounter_id,
            condition_value_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Encounter condition")?;

        Ok(())
    }
}
