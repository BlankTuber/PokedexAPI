use crate::error::ApiResult;
use crate::models::location::{CreateLocation, Location, UpdateLocation};
use crate::repositories::location_repository::LocationRepository;
use crate::validators::location_validator::LocationValidator;
use sqlx::PgPool;

pub struct LocationService;

impl LocationService {
    pub async fn create(pool: &PgPool, data: CreateLocation) -> ApiResult<Location> {
        LocationValidator::validate_create(&data)?;
        LocationRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<Location> {
        LocationRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<Location>> {
        LocationRepository::find_all(pool).await
    }

    pub async fn list_by_region(pool: &PgPool, region_id: i32) -> ApiResult<Vec<Location>> {
        LocationRepository::find_by_region(pool, region_id).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateLocation) -> ApiResult<()> {
        LocationValidator::validate_update(&data)?;
        LocationRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        LocationRepository::delete(pool, id).await
    }
}
