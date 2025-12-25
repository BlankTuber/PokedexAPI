use crate::error::ApiResult;
use crate::models::pokemon_type::{CreatePokemonType, PokemonType, UpdatePokemonType};
use crate::repositories::pokemon_type_repository::PokemonTypeRepository;
use crate::validators::pokemon_type_validator::PokemonTypeValidator;
use sqlx::PgPool;

pub struct PokemonTypeService;

impl PokemonTypeService {
    pub async fn create(pool: &PgPool, data: CreatePokemonType) -> ApiResult<PokemonType> {
        PokemonTypeValidator::validate_create(&data)?;
        PokemonTypeRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<PokemonType> {
        PokemonTypeRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokemonType>> {
        PokemonTypeRepository::find_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdatePokemonType) -> ApiResult<()> {
        PokemonTypeValidator::validate_update(&data)?;
        PokemonTypeRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        PokemonTypeRepository::delete(pool, id).await
    }
}
