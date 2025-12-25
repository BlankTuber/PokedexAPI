use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::platform::{CreatePlatform, Platform, UpdatePlatform},
    repositories::platform_repository::PlatformRepository,
    validators::platform_validator::PlatformValidator,
};

pub struct PlatformService;

impl PlatformService {
    pub async fn create_platform(pool: &PgPool, data: CreatePlatform) -> ApiResult<Platform> {
        PlatformValidator::validate_create(&data)?;
        PlatformRepository::create(pool, data).await
    }

    pub async fn get_platform(pool: &PgPool, id: i32) -> ApiResult<Platform> {
        PlatformRepository::find_by_id(pool, id).await
    }

    pub async fn list_platforms(pool: &PgPool) -> ApiResult<Vec<Platform>> {
        PlatformRepository::find_all(pool).await
    }

    pub async fn update_platform(pool: &PgPool, id: i32, data: UpdatePlatform) -> ApiResult<()> {
        PlatformValidator::validate_update(&data)?;
        PlatformRepository::update(pool, id, data).await
    }

    pub async fn delete_platform(pool: &PgPool, id: i32) -> ApiResult<()> {
        PlatformRepository::delete(pool, id).await
    }
}
