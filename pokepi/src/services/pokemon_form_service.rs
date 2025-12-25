use crate::error::ApiResult;
use crate::models::pokemon_form::{CreatePokemonForm, PokemonForm, UpdatePokemonForm};
use crate::repositories::pokemon_form_repository::PokemonFormRepository;
use crate::validators::pokemon_form_validator::PokemonFormValidator;
use sqlx::PgPool;

pub struct PokemonFormService;

impl PokemonFormService {
    pub async fn create_pokemon_form(pool: &PgPool, data: CreatePokemonForm) -> ApiResult<PokemonForm> {
        PokemonFormValidator::validate_create(&data)?;
        PokemonFormRepository::create(pool, data).await
    }

    pub async fn get_pokemon_form(pool: &PgPool, form_id: i32) -> ApiResult<PokemonForm> {
        PokemonFormRepository::find_by_id(pool, form_id).await
    }

    pub async fn list_pokemon_forms(pool: &PgPool) -> ApiResult<Vec<PokemonForm>> {
        PokemonFormRepository::find_all(pool).await
    }

    pub async fn list_forms_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokemonForm>> {
        PokemonFormRepository::find_by_pokemon(pool, national_id).await
    }

    pub async fn update_pokemon_form(pool: &PgPool, form_id: i32, data: UpdatePokemonForm) -> ApiResult<()> {
        PokemonFormValidator::validate_update(&data)?;
        PokemonFormRepository::update(pool, form_id, data).await
    }

    pub async fn delete_pokemon_form(pool: &PgPool, form_id: i32) -> ApiResult<()> {
        PokemonFormRepository::delete(pool, form_id).await
    }
}
