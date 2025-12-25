use crate::error::ApiResult;
use crate::models::pokemon::{CreatePokemon, Pokemon, UpdatePokemon};
use crate::services::pokemon_service::PokemonService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/pokemon", format = "json", data = "<data>")]
pub async fn create_pokemon(
    pool: &State<PgPool>,
    data: Json<CreatePokemon>,
) -> ApiResult<Json<Pokemon>> {
    let pokemon = PokemonService::create_pokemon(pool.inner(), data.into_inner()).await?;
    Ok(Json(pokemon))
}

#[get("/pokemon/<national_id>")]
pub async fn get_pokemon(pool: &State<PgPool>, national_id: i32) -> ApiResult<Json<Pokemon>> {
    let pokemon = PokemonService::get_pokemon(pool.inner(), national_id).await?;
    Ok(Json(pokemon))
}

#[get("/pokemon")]
pub async fn list_pokemon(pool: &State<PgPool>) -> ApiResult<Json<Vec<Pokemon>>> {
    let pokemon = PokemonService::list_pokemon(pool.inner()).await?;
    Ok(Json(pokemon))
}

#[patch("/pokemon/<national_id>", format = "json", data = "<data>")]
pub async fn update_pokemon(
    pool: &State<PgPool>,
    national_id: i32,
    data: Json<UpdatePokemon>,
) -> ApiResult<Json<Pokemon>> {
    PokemonService::update_pokemon(pool.inner(), national_id, data.into_inner()).await?;
    let pokemon = PokemonService::get_pokemon(pool.inner(), national_id).await?;
    Ok(Json(pokemon))
}

#[delete("/pokemon/<national_id>")]
pub async fn delete_pokemon(pool: &State<PgPool>, national_id: i32) -> ApiResult<Status> {
    PokemonService::delete_pokemon(pool.inner(), national_id).await?;
    Ok(Status::NoContent)
}

pub fn pokemon_routes() -> Vec<Route> {
    routes![
        create_pokemon,
        get_pokemon,
        list_pokemon,
        update_pokemon,
        delete_pokemon
    ]
}
