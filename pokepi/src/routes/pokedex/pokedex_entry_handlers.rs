use crate::error::ApiResult;
use crate::models::pokedex_entry::{CreatePokedexEntry, PokedexEntry, UpdatePokedexEntry};
use crate::services::pokedex_entry_service::PokedexEntryService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/pokedex-entries", format = "json", data = "<data>")]
pub async fn create_pokedex_entry(
    pool: &State<PgPool>,
    data: Json<CreatePokedexEntry>,
) -> ApiResult<Json<PokedexEntry>> {
    let result = PokedexEntryService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/pokemon/<national_id>/entries/<game_id>")]
pub async fn get_pokedex_entry_base_form(
    pool: &State<PgPool>,
    national_id: i32,
    game_id: i32,
) -> ApiResult<Json<PokedexEntry>> {
    let result = PokedexEntryService::get(pool.inner(), national_id, None, game_id).await?;
    Ok(Json(result))
}

#[get("/pokemon/<national_id>/forms/<form_id>/entries/<game_id>")]
pub async fn get_pokedex_entry_with_form(
    pool: &State<PgPool>,
    national_id: i32,
    form_id: i32,
    game_id: i32,
) -> ApiResult<Json<PokedexEntry>> {
    let result = PokedexEntryService::get(pool.inner(), national_id, Some(form_id), game_id).await?;
    Ok(Json(result))
}

#[get("/pokedex-entries")]
pub async fn list_pokedex_entries(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokedexEntry>>> {
    let results = PokedexEntryService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/pokemon/<national_id>/entries")]
pub async fn list_entries_by_pokemon(
    pool: &State<PgPool>,
    national_id: i32,
) -> ApiResult<Json<Vec<PokedexEntry>>> {
    let results = PokedexEntryService::list_by_pokemon(pool.inner(), national_id).await?;
    Ok(Json(results))
}

#[get("/games/<game_id>/pokedex-entries")]
pub async fn list_entries_by_game(
    pool: &State<PgPool>,
    game_id: i32,
) -> ApiResult<Json<Vec<PokedexEntry>>> {
    let results = PokedexEntryService::list_by_game(pool.inner(), game_id).await?;
    Ok(Json(results))
}

#[patch("/pokemon/<national_id>/entries/<game_id>", format = "json", data = "<data>")]
pub async fn update_pokedex_entry_base_form(
    pool: &State<PgPool>,
    national_id: i32,
    game_id: i32,
    data: Json<UpdatePokedexEntry>,
) -> ApiResult<Json<PokedexEntry>> {
    PokedexEntryService::update(pool.inner(), national_id, None, game_id, data.into_inner()).await?;
    let result = PokedexEntryService::get(pool.inner(), national_id, None, game_id).await?;
    Ok(Json(result))
}

#[patch("/pokemon/<national_id>/forms/<form_id>/entries/<game_id>", format = "json", data = "<data>")]
pub async fn update_pokedex_entry_with_form(
    pool: &State<PgPool>,
    national_id: i32,
    form_id: i32,
    game_id: i32,
    data: Json<UpdatePokedexEntry>,
) -> ApiResult<Json<PokedexEntry>> {
    PokedexEntryService::update(pool.inner(), national_id, Some(form_id), game_id, data.into_inner()).await?;
    let result = PokedexEntryService::get(pool.inner(), national_id, Some(form_id), game_id).await?;
    Ok(Json(result))
}

#[delete("/pokemon/<national_id>/entries/<game_id>")]
pub async fn delete_pokedex_entry_base_form(
    pool: &State<PgPool>,
    national_id: i32,
    game_id: i32,
) -> ApiResult<Status> {
    PokedexEntryService::delete(pool.inner(), national_id, None, game_id).await?;
    Ok(Status::NoContent)
}

#[delete("/pokemon/<national_id>/forms/<form_id>/entries/<game_id>")]
pub async fn delete_pokedex_entry_with_form(
    pool: &State<PgPool>,
    national_id: i32,
    form_id: i32,
    game_id: i32,
) -> ApiResult<Status> {
    PokedexEntryService::delete(pool.inner(), national_id, Some(form_id), game_id).await?;
    Ok(Status::NoContent)
}

pub fn pokedex_entry_routes() -> Vec<Route> {
    routes![
        create_pokedex_entry,
        get_pokedex_entry_base_form,
        get_pokedex_entry_with_form,
        list_pokedex_entries,
        list_entries_by_pokemon,
        list_entries_by_game,
        update_pokedex_entry_base_form,
        update_pokedex_entry_with_form,
        delete_pokedex_entry_base_form,
        delete_pokedex_entry_with_form
    ]
}
