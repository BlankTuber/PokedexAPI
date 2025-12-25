use crate::error::ApiResult;
use crate::models::encounter_condition::{CreateEncounterCondition, EncounterCondition};
use crate::repositories::encounter_condition_repository::EncounterConditionRepository;
use crate::validators::encounter_condition_validator::EncounterConditionValidator;
use sqlx::PgPool;

pub struct EncounterConditionService;

impl EncounterConditionService {
    pub async fn create(pool: &PgPool, data: CreateEncounterCondition) -> ApiResult<EncounterCondition> {
        EncounterConditionValidator::validate_create(&data)?;
        EncounterConditionRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, encounter_id: i32, condition_value_id: i32) -> ApiResult<EncounterCondition> {
        EncounterConditionRepository::find_by_key(pool, encounter_id, condition_value_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<EncounterCondition>> {
        EncounterConditionRepository::find_all(pool).await
    }

    pub async fn list_by_encounter(pool: &PgPool, encounter_id: i32) -> ApiResult<Vec<EncounterCondition>> {
        EncounterConditionRepository::find_by_encounter(pool, encounter_id).await
    }

    pub async fn delete(pool: &PgPool, encounter_id: i32, condition_value_id: i32) -> ApiResult<()> {
        EncounterConditionRepository::delete(pool, encounter_id, condition_value_id).await
    }
}
