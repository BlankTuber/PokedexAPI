use sqlx::PgPool;

use crate::error::ApiResult;
use crate::models::pokedex_number::{CreatePokedexNumber, PokedexNumber, UpdatePokedexNumber};
use crate::repositories::pokedex_number_repository::PokedexNumberRepository;
use crate::validators::common::CommonValidator;
use crate::validators::pokedex_number_validator::PokedexNumberValidator;

pub struct PokedexNumberService;

impl PokedexNumberService {
    pub async fn create(pool: &PgPool, data: CreatePokedexNumber) -> ApiResult<PokedexNumber> {
        PokedexNumberValidator::validate_create(&data)?;
        PokedexNumberRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, national_id: i32, pokedex_id: i32) -> ApiResult<PokedexNumber> {
        PokedexNumberRepository::get(pool, national_id, pokedex_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokedexNumber>> {
        PokedexNumberRepository::list(pool).await
    }

    pub async fn list_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokedexNumber>> {
        PokedexNumberRepository::list_by_pokemon(pool, national_id).await
    }

    pub async fn list_by_pokedex(pool: &PgPool, pokedex_id: i32) -> ApiResult<Vec<PokedexNumber>> {
        PokedexNumberRepository::list_by_pokedex(pool, pokedex_id).await
    }

    pub async fn update(
        pool: &PgPool,
        national_id: i32,
        pokedex_id: i32,
        data: UpdatePokedexNumber,
    ) -> ApiResult<()> {
        PokedexNumberValidator::validate_update(&data)?;
        let rows = PokedexNumberRepository::update(pool, national_id, pokedex_id, data).await?;
        CommonValidator::validate_rows_affected(rows, "Pokedex number")
    }

    pub async fn delete(pool: &PgPool, national_id: i32, pokedex_id: i32) -> ApiResult<()> {
        let rows = PokedexNumberRepository::delete(pool, national_id, pokedex_id).await?;
        CommonValidator::validate_rows_affected(rows, "Pokedex number")
    }
}
