use crate::error::ApiResult;
use crate::models::pokemon_form_type::{CreatePokemonFormType, PokemonFormType, UpdatePokemonFormType};
use crate::repositories::pokemon_form_type_repository::PokemonFormTypeRepository;
use crate::validators::pokemon_form_type_validator::PokemonFormTypeValidator;
use sqlx::PgPool;

pub struct PokemonFormTypeService;

impl PokemonFormTypeService {
    pub async fn create(pool: &PgPool, data: CreatePokemonFormType) -> ApiResult<PokemonFormType> {
        PokemonFormTypeValidator::validate_create(&data)?;
        PokemonFormTypeRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, pokemon_form_game_id: i32, type_id: i32) -> ApiResult<PokemonFormType> {
        PokemonFormTypeRepository::find_by_key(pool, pokemon_form_game_id, type_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokemonFormType>> {
        PokemonFormTypeRepository::find_all(pool).await
    }

    pub async fn list_by_form_game(pool: &PgPool, pokemon_form_game_id: i32) -> ApiResult<Vec<PokemonFormType>> {
        PokemonFormTypeRepository::find_by_form_game(pool, pokemon_form_game_id).await
    }

    pub async fn update(pool: &PgPool, pokemon_form_game_id: i32, type_id: i32, data: UpdatePokemonFormType) -> ApiResult<()> {
        PokemonFormTypeValidator::validate_update(&data)?;
        PokemonFormTypeRepository::update(pool, pokemon_form_game_id, type_id, data).await
    }

    pub async fn delete(pool: &PgPool, pokemon_form_game_id: i32, type_id: i32) -> ApiResult<()> {
        PokemonFormTypeRepository::delete(pool, pokemon_form_game_id, type_id).await
    }
}
