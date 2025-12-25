use crate::error::ApiResult;
use crate::models::location_area::{CreateLocationArea, LocationArea, UpdateLocationArea};
use crate::repositories::location_area_repository::LocationAreaRepository;
use crate::validators::location_area_validator::LocationAreaValidator;
use sqlx::PgPool;

pub struct LocationAreaService;

impl LocationAreaService {
    pub async fn create(pool: &PgPool, data: CreateLocationArea) -> ApiResult<LocationArea> {
        LocationAreaValidator::validate_create(&data)?;
        LocationAreaRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<LocationArea> {
        LocationAreaRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<LocationArea>> {
        LocationAreaRepository::find_all(pool).await
    }

    pub async fn list_by_location(pool: &PgPool, location_id: i32) -> ApiResult<Vec<LocationArea>> {
        LocationAreaRepository::find_by_location(pool, location_id).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateLocationArea) -> ApiResult<()> {
        LocationAreaValidator::validate_update(&data)?;
        LocationAreaRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        LocationAreaRepository::delete(pool, id).await
    }
}
