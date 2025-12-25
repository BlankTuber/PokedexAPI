use crate::error::ApiResult;
use crate::models::pokemon::{CreatePokemon, Pokemon, UpdatePokemon};
use crate::repositories::pokemon_repository::PokemonRepository;
use crate::validators::pokemon_validator::PokemonValidator;
use sqlx::PgPool;

pub struct PokemonService;

impl PokemonService {
    pub async fn create_pokemon(pool: &PgPool, data: CreatePokemon) -> ApiResult<Pokemon> {
        PokemonValidator::validate_create(&data)?;
        PokemonRepository::create(pool, data).await
    }

    pub async fn get_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Pokemon> {
        PokemonRepository::find_by_id(pool, national_id).await
    }

    pub async fn list_pokemon(pool: &PgPool) -> ApiResult<Vec<Pokemon>> {
        PokemonRepository::find_all(pool).await
    }

    pub async fn update_pokemon(pool: &PgPool, national_id: i32, data: UpdatePokemon) -> ApiResult<()> {
        PokemonValidator::validate_update(&data)?;
        PokemonRepository::update(pool, national_id, data).await
    }

    pub async fn delete_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<()> {
        PokemonRepository::delete(pool, national_id).await
    }
}
