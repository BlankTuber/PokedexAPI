use crate::error::ApiResult;
use crate::models::pokemon_type::{CreatePokemonType, PokemonType, UpdatePokemonType};
use crate::services::pokemon_type_service::PokemonTypeService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/types", format = "json", data = "<data>")]
pub async fn create_pokemon_type(
    pool: &State<PgPool>,
    data: Json<CreatePokemonType>,
) -> ApiResult<Json<PokemonType>> {
    let result = PokemonTypeService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/types/<id>")]
pub async fn get_pokemon_type(pool: &State<PgPool>, id: i32) -> ApiResult<Json<PokemonType>> {
    let result = PokemonTypeService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/types")]
pub async fn list_pokemon_types(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokemonType>>> {
    let results = PokemonTypeService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[patch("/types/<id>", format = "json", data = "<data>")]
pub async fn update_pokemon_type(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdatePokemonType>,
) -> ApiResult<Json<PokemonType>> {
    PokemonTypeService::update(pool.inner(), id, data.into_inner()).await?;
    let result = PokemonTypeService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/types/<id>")]
pub async fn delete_pokemon_type(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    PokemonTypeService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn pokemon_type_routes() -> Vec<Route> {
    routes![
        create_pokemon_type,
        get_pokemon_type,
        list_pokemon_types,
        update_pokemon_type,
        delete_pokemon_type
    ]
}
