use crate::error::ApiResult;
use crate::models::region::{CreateRegion, Region, UpdateRegion};
use crate::repositories::region_repository::RegionRepository;
use crate::validators::region_validator::RegionValidator;
use sqlx::PgPool;

pub struct RegionService;

impl RegionService {
    pub async fn create(pool: &PgPool, data: CreateRegion) -> ApiResult<Region> {
        RegionValidator::validate_create(&data)?;
        RegionRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<Region> {
        RegionRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<Region>> {
        RegionRepository::find_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateRegion) -> ApiResult<()> {
        RegionValidator::validate_update(&data)?;
        RegionRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        RegionRepository::delete(pool, id).await
    }
}
