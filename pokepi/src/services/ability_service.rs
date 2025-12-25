use crate::error::ApiResult;
use crate::models::ability::{Ability, CreateAbility, UpdateAbility};
use crate::repositories::ability_repository::AbilityRepository;
use crate::validators::ability_validator::AbilityValidator;
use sqlx::PgPool;

pub struct AbilityService;

impl AbilityService {
    pub async fn create(pool: &PgPool, data: CreateAbility) -> ApiResult<Ability> {
        AbilityValidator::validate_create(&data)?;
        AbilityRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<Ability> {
        AbilityRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<Ability>> {
        AbilityRepository::find_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateAbility) -> ApiResult<()> {
        AbilityValidator::validate_update(&data)?;
        AbilityRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        AbilityRepository::delete(pool, id).await
    }
}
