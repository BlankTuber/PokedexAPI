use crate::error::ApiResult;
use crate::models::encounter_condition_values::{CreateEncounterConditionValue, EncounterConditionValue, UpdateEncounterConditionValue};
use crate::repositories::encounter_condition_value_repository::EncounterConditionValueRepository;
use crate::validators::encounter_condition_value_validator::EncounterConditionValueValidator;
use sqlx::PgPool;

pub struct EncounterConditionValueService;

impl EncounterConditionValueService {
    pub async fn create(pool: &PgPool, data: CreateEncounterConditionValue) -> ApiResult<EncounterConditionValue> {
        EncounterConditionValueValidator::validate_create(&data)?;
        EncounterConditionValueRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<EncounterConditionValue> {
        EncounterConditionValueRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<EncounterConditionValue>> {
        EncounterConditionValueRepository::find_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEncounterConditionValue) -> ApiResult<()> {
        EncounterConditionValueValidator::validate_update(&data)?;
        EncounterConditionValueRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        EncounterConditionValueRepository::delete(pool, id).await
    }
}
