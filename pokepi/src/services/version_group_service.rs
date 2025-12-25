use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::version_group::{CreateVersionGroup, UpdateVersionGroup, VersionGroup},
    repositories::version_group_repository::VersionGroupRepository,
    validators::version_group_validator::VersionGroupValidator,
};

pub struct VersionGroupService;

impl VersionGroupService {
    pub async fn create_version_group(
        pool: &PgPool,
        data: CreateVersionGroup,
    ) -> ApiResult<VersionGroup> {
        VersionGroupValidator::validate_create(&data)?;
        VersionGroupRepository::create(pool, data).await
    }

    pub async fn get_version_group(pool: &PgPool, id: i32) -> ApiResult<VersionGroup> {
        VersionGroupRepository::find_by_id(pool, id).await
    }

    pub async fn list_version_groups(pool: &PgPool) -> ApiResult<Vec<VersionGroup>> {
        VersionGroupRepository::find_all(pool).await
    }

    pub async fn update_version_group(
        pool: &PgPool,
        id: i32,
        data: UpdateVersionGroup,
    ) -> ApiResult<()> {
        VersionGroupValidator::validate_update(&data)?;
        VersionGroupRepository::update(pool, id, data).await
    }

    pub async fn delete_version_group(pool: &PgPool, id: i32) -> ApiResult<()> {
        VersionGroupRepository::delete(pool, id).await
    }
}
