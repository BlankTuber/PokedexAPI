use crate::error::ApiResult;
use crate::models::relation_group::{CreateRelationGroup, RelationGroup, UpdateRelationGroup};
use crate::repositories::relation_group_repository::RelationGroupRepository;
use crate::validators::relation_group_validator::RelationGroupValidator;
use sqlx::PgPool;

pub struct RelationGroupService;

impl RelationGroupService {
    pub async fn create(pool: &PgPool, data: CreateRelationGroup) -> ApiResult<RelationGroup> {
        RelationGroupValidator::validate_create(&data)?;
        RelationGroupRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<RelationGroup> {
        RelationGroupRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<RelationGroup>> {
        RelationGroupRepository::find_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateRelationGroup) -> ApiResult<()> {
        RelationGroupValidator::validate_update(&data)?;
        RelationGroupRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        RelationGroupRepository::delete(pool, id).await
    }
}
