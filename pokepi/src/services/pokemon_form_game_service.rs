use crate::error::ApiResult;
use crate::models::pokemon_form_game::{CreatePokemonFormGame, PokemonFormGame, UpdatePokemonFormGame};
use crate::repositories::pokemon_form_game_repository::PokemonFormGameRepository;
use crate::validators::pokemon_form_game_validator::PokemonFormGameValidator;
use sqlx::PgPool;

pub struct PokemonFormGameService;

impl PokemonFormGameService {
    pub async fn create(pool: &PgPool, data: CreatePokemonFormGame) -> ApiResult<PokemonFormGame> {
        PokemonFormGameValidator::validate_create(&data)?;
        PokemonFormGameRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, id: i32) -> ApiResult<PokemonFormGame> {
        PokemonFormGameRepository::find_by_id(pool, id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokemonFormGame>> {
        PokemonFormGameRepository::find_all(pool).await
    }

    pub async fn list_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokemonFormGame>> {
        PokemonFormGameRepository::find_by_pokemon(pool, national_id).await
    }

    pub async fn list_by_form(pool: &PgPool, form_id: i32) -> ApiResult<Vec<PokemonFormGame>> {
        PokemonFormGameRepository::find_by_form(pool, form_id).await
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdatePokemonFormGame) -> ApiResult<()> {
        PokemonFormGameValidator::validate_update(&data)?;
        PokemonFormGameRepository::update(pool, id, data).await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        PokemonFormGameRepository::delete(pool, id).await
    }
}
