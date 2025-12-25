use crate::error::ApiResult;
use crate::models::type_matchup::{CreateTypeMatchup, TypeMatchup, UpdateTypeMatchup};
use crate::repositories::type_matchup_repository::TypeMatchupRepository;
use crate::validators::type_matchup_validator::TypeMatchupValidator;
use sqlx::PgPool;

pub struct TypeMatchupService;

impl TypeMatchupService {
    pub async fn create(pool: &PgPool, data: CreateTypeMatchup) -> ApiResult<TypeMatchup> {
        TypeMatchupValidator::validate_create(&data)?;
        TypeMatchupRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, attacking_type_id: i32, defending_type_id: i32) -> ApiResult<TypeMatchup> {
        TypeMatchupRepository::find_by_key(pool, attacking_type_id, defending_type_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<TypeMatchup>> {
        TypeMatchupRepository::find_all(pool).await
    }

    pub async fn list_by_attacking_type(pool: &PgPool, attacking_type_id: i32) -> ApiResult<Vec<TypeMatchup>> {
        TypeMatchupRepository::find_by_attacking_type(pool, attacking_type_id).await
    }

    pub async fn list_by_defending_type(pool: &PgPool, defending_type_id: i32) -> ApiResult<Vec<TypeMatchup>> {
        TypeMatchupRepository::find_by_defending_type(pool, defending_type_id).await
    }

    pub async fn update(pool: &PgPool, attacking_type_id: i32, defending_type_id: i32, data: UpdateTypeMatchup) -> ApiResult<()> {
        TypeMatchupValidator::validate_update(&data)?;
        TypeMatchupRepository::update(pool, attacking_type_id, defending_type_id, data).await
    }

    pub async fn delete(pool: &PgPool, attacking_type_id: i32, defending_type_id: i32) -> ApiResult<()> {
        TypeMatchupRepository::delete(pool, attacking_type_id, defending_type_id).await
    }
}
