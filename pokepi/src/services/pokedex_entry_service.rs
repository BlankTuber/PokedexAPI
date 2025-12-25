use sqlx::PgPool;

use crate::error::ApiResult;
use crate::models::pokedex_entry::{CreatePokedexEntry, PokedexEntry, UpdatePokedexEntry};
use crate::repositories::pokedex_entry_repository::PokedexEntryRepository;
use crate::validators::common::CommonValidator;
use crate::validators::pokedex_entry_validator::PokedexEntryValidator;

pub struct PokedexEntryService;

impl PokedexEntryService {
    pub async fn create(pool: &PgPool, data: CreatePokedexEntry) -> ApiResult<PokedexEntry> {
        PokedexEntryValidator::validate_create(&data)?;
        PokedexEntryRepository::create(pool, data).await
    }

    pub async fn get(
        pool: &PgPool,
        national_id: i32,
        form_id: Option<i32>,
        game_id: i32,
    ) -> ApiResult<PokedexEntry> {
        PokedexEntryRepository::get(pool, national_id, form_id, game_id).await
    }

    pub async fn list(pool: &PgPool) -> ApiResult<Vec<PokedexEntry>> {
        PokedexEntryRepository::list(pool).await
    }

    pub async fn list_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokedexEntry>> {
        PokedexEntryRepository::list_by_pokemon(pool, national_id).await
    }

    pub async fn list_by_game(pool: &PgPool, game_id: i32) -> ApiResult<Vec<PokedexEntry>> {
        PokedexEntryRepository::list_by_game(pool, game_id).await
    }

    pub async fn update(
        pool: &PgPool,
        national_id: i32,
        form_id: Option<i32>,
        game_id: i32,
        data: UpdatePokedexEntry,
    ) -> ApiResult<()> {
        PokedexEntryValidator::validate_update(&data)?;
        let rows = PokedexEntryRepository::update(pool, national_id, form_id, game_id, data).await?;
        CommonValidator::validate_rows_affected(rows, "Pokedex entry")
    }

    pub async fn delete(
        pool: &PgPool,
        national_id: i32,
        form_id: Option<i32>,
        game_id: i32,
    ) -> ApiResult<()> {
        let rows = PokedexEntryRepository::delete(pool, national_id, form_id, game_id).await?;
        CommonValidator::validate_rows_affected(rows, "Pokedex entry")
    }
}
