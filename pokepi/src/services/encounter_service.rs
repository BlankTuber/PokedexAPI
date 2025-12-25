use crate::error::ApiResult;
use crate::models::encounter::{CreateEncounter, Encounter, UpdateEncounter};
use crate::repositories::encounter_repository::EncounterRepository;
use crate::validators::encounter_validator::EncounterValidator;
use sqlx::PgPool;

pub struct EncounterService;

impl EncounterService {
    pub async fn create(pool: &PgPool, data: CreateEncounter) -> ApiResult<Encounter> {
        EncounterValidator::validate_create(&data)?;
        EncounterRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<Encounter> {
        EncounterRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<Encounter>> {
        EncounterRepository::find_all(pool).await
    }

    pub async fn list_by_location_area(pool: &PgPool, location_area_id: i32) -> ApiResult<Vec<Encounter>> {
        EncounterRepository::find_by_location_area(pool, location_area_id).await
    }

    pub async fn list_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<Encounter>> {
        EncounterRepository::find_by_pokemon(pool, national_id).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEncounter) -> ApiResult<()> {
        EncounterValidator::validate_update(&data)?;
        EncounterRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        EncounterRepository::delete(pool, id).await
    }
}
