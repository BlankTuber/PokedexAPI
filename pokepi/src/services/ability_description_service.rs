use crate::error::ApiResult;
use crate::models::ability_description::{AbilityDescription, CreateAbilityDescription, UpdateAbilityDescription};
use crate::repositories::ability_description_repository::AbilityDescriptionRepository;
use crate::validators::ability_description_validator::AbilityDescriptionValidator;
use sqlx::PgPool;

pub struct AbilityDescriptionService;

impl AbilityDescriptionService {
    pub async fn create(pool: &PgPool, data: CreateAbilityDescription) -> ApiResult<AbilityDescription> {
        AbilityDescriptionValidator::validate_create(&data)?;
        AbilityDescriptionRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, ability_id: i32, version_group_id: i32) -> ApiResult<AbilityDescription> {
        AbilityDescriptionRepository::find_by_key(pool, ability_id, version_group_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<AbilityDescription>> {
        AbilityDescriptionRepository::find_all(pool).await
    }

    pub async fn list_by_ability(pool: &PgPool, ability_id: i32) -> ApiResult<Vec<AbilityDescription>> {
        AbilityDescriptionRepository::find_by_ability(pool, ability_id).await
    }

    pub async fn update(pool: &PgPool, ability_id: i32, version_group_id: i32, data: UpdateAbilityDescription) -> ApiResult<()> {
        AbilityDescriptionValidator::validate_update(&data)?;
        AbilityDescriptionRepository::update(pool, ability_id, version_group_id, data).await
    }

    pub async fn delete(pool: &PgPool, ability_id: i32, version_group_id: i32) -> ApiResult<()> {
        AbilityDescriptionRepository::delete(pool, ability_id, version_group_id).await
    }
}
