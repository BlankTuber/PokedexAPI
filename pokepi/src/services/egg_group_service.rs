use crate::error::ApiResult;
use crate::models::egg_group::{CreateEggGroup, EggGroup, UpdateEggGroup};
use crate::repositories::egg_group_repository::EggGroupRepository;
use crate::validators::egg_group_validator::EggGroupValidator;
use sqlx::PgPool;

pub struct EggGroupService;

impl EggGroupService {
    pub async fn create_egg_group(pool: &PgPool, data: CreateEggGroup) -> ApiResult<EggGroup> {
        EggGroupValidator::validate_create(&data)?;
        EggGroupRepository::create(pool, data).await
    }

    pub async fn get_egg_group(pool: &PgPool, id: i32) -> ApiResult<EggGroup> {
        EggGroupRepository::find_by_id(pool, id).await
    }

    pub async fn list_egg_groups(pool: &PgPool) -> ApiResult<Vec<EggGroup>> {
        EggGroupRepository::find_all(pool).await
    }

    pub async fn update_egg_group(pool: &PgPool, id: i32, data: UpdateEggGroup) -> ApiResult<()> {
        EggGroupValidator::validate_update(&data)?;
        EggGroupRepository::update(pool, id, data).await
    }

    pub async fn delete_egg_group(pool: &PgPool, id: i32) -> ApiResult<()> {
        EggGroupRepository::delete(pool, id).await
    }
}
