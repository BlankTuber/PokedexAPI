use crate::error::ApiResult;
use crate::models::pokemon_form_game::{CreatePokemonFormGame, PokemonFormGame, UpdatePokemonFormGame};
use crate::services::pokemon_form_game_service::PokemonFormGameService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/form-games", format = "json", data = "<data>")]
pub async fn create_pokemon_form_game(
    pool: &State<PgPool>,
    data: Json<CreatePokemonFormGame>,
) -> ApiResult<Json<PokemonFormGame>> {
    let result = PokemonFormGameService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/form-games/<id>")]
pub async fn get_pokemon_form_game(pool: &State<PgPool>, id: i32) -> ApiResult<Json<PokemonFormGame>> {
    let result = PokemonFormGameService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/form-games")]
pub async fn list_pokemon_form_games(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokemonFormGame>>> {
    let results = PokemonFormGameService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/pokemon/<national_id>/form-games")]
pub async fn list_form_games_by_pokemon(pool: &State<PgPool>, national_id: i32) -> ApiResult<Json<Vec<PokemonFormGame>>> {
    let results = PokemonFormGameService::list_by_pokemon(pool.inner(), national_id).await?;
    Ok(Json(results))
}

#[get("/forms/<form_id>/games")]
pub async fn list_form_games_by_form(pool: &State<PgPool>, form_id: i32) -> ApiResult<Json<Vec<PokemonFormGame>>> {
    let results = PokemonFormGameService::list_by_form(pool.inner(), form_id).await?;
    Ok(Json(results))
}

#[patch("/form-games/<id>", format = "json", data = "<data>")]
pub async fn update_pokemon_form_game(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdatePokemonFormGame>,
) -> ApiResult<Json<PokemonFormGame>> {
    PokemonFormGameService::update(pool.inner(), id, data.into_inner()).await?;
    let result = PokemonFormGameService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/form-games/<id>")]
pub async fn delete_pokemon_form_game(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    PokemonFormGameService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn pokemon_form_game_routes() -> Vec<Route> {
    routes![
        create_pokemon_form_game,
        get_pokemon_form_game,
        list_pokemon_form_games,
        list_form_games_by_pokemon,
        list_form_games_by_form,
        update_pokemon_form_game,
        delete_pokemon_form_game
    ]
}
