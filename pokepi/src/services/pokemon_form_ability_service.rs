use crate::error::ApiResult;
use crate::models::pokemon_form_ability::{CreatePokemonFormAbility, PokemonFormAbility, UpdatePokemonFormAbility};
use crate::repositories::pokemon_form_ability_repository::PokemonFormAbilityRepository;
use crate::validators::pokemon_form_ability_validator::PokemonFormAbilityValidator;
use sqlx::PgPool;

pub struct PokemonFormAbilityService;

impl PokemonFormAbilityService {
    pub async fn create(pool: &PgPool, data: CreatePokemonFormAbility) -> ApiResult<PokemonFormAbility> {
        PokemonFormAbilityValidator::validate_create(&data)?;
        PokemonFormAbilityRepository::create(pool, data).await
    }

    pub async fn get(pool: &PgPool, pokemon_form_game_id: i32, ability_id: i32) -> ApiResult<PokemonFormAbility> {
        PokemonFormAbilityRepository::find_by_key(pool, pokemon_form_game_id, ability_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokemonFormAbility>> {
        PokemonFormAbilityRepository::find_all(pool).await
    }

    pub async fn list_by_form_game(pool: &PgPool, pokemon_form_game_id: i32) -> ApiResult<Vec<PokemonFormAbility>> {
        PokemonFormAbilityRepository::find_by_form_game(pool, pokemon_form_game_id).await
    }

    pub async fn update(pool: &PgPool, pokemon_form_game_id: i32, ability_id: i32, data: UpdatePokemonFormAbility) -> ApiResult<()> {
        PokemonFormAbilityValidator::validate_update(&data)?;
        PokemonFormAbilityRepository::update(pool, pokemon_form_game_id, ability_id, data).await
    }

    pub async fn delete(pool: &PgPool, pokemon_form_game_id: i32, ability_id: i32) -> ApiResult<()> {
        PokemonFormAbilityRepository::delete(pool, pokemon_form_game_id, ability_id).await
    }
}
