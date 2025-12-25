use crate::error::ApiResult;
use crate::models::encounter_method::{CreateEncounterMethod, EncounterMethod, UpdateEncounterMethod};
use crate::repositories::encounter_method_repository::EncounterMethodRepository;
use crate::validators::encounter_method_validator::EncounterMethodValidator;
use sqlx::PgPool;

pub struct EncounterMethodService;

impl EncounterMethodService {
    pub async fn create(pool: &PgPool, data: CreateEncounterMethod) -> ApiResult<EncounterMethod> {
        EncounterMethodValidator::validate_create(&data)?;
        EncounterMethodRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<EncounterMethod> {
        EncounterMethodRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<EncounterMethod>> {
        EncounterMethodRepository::find_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEncounterMethod) -> ApiResult<()> {
        EncounterMethodValidator::validate_update(&data)?;
        EncounterMethodRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        EncounterMethodRepository::delete(pool, id).await
    }
}
