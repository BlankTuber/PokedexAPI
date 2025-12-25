use crate::error::ApiResult;
use crate::models::regional_pokedex::{CreateRegionalPokedex, RegionalPokedex, UpdateRegionalPokedex};
use crate::repositories::regional_pokedex_repository::RegionalPokedexRepository;
use crate::validators::regional_pokedex_validator::RegionalPokedexValidator;
use sqlx::PgPool;

pub struct RegionalPokedexService;

impl RegionalPokedexService {
    pub async fn create(pool: &PgPool, data: CreateRegionalPokedex) -> ApiResult<RegionalPokedex> {
        RegionalPokedexValidator::validate_create(&data)?;
        RegionalPokedexRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<RegionalPokedex> {
        RegionalPokedexRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<RegionalPokedex>> {
        RegionalPokedexRepository::find_all(pool).await
    }

    pub async fn list_by_region(pool: &PgPool, region_id: i32) -> ApiResult<Vec<RegionalPokedex>> {
        RegionalPokedexRepository::find_by_region(pool, region_id).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateRegionalPokedex) -> ApiResult<()> {
        RegionalPokedexValidator::validate_update(&data)?;
        RegionalPokedexRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        RegionalPokedexRepository::delete(pool, id).await
    }
}
