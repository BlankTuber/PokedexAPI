use crate::error::ApiResult;
use crate::models::pokemon_form_type::{CreatePokemonFormType, PokemonFormType, UpdatePokemonFormType};
use crate::services::pokemon_form_type_service::PokemonFormTypeService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/form-types", format = "json", data = "<data>")]
pub async fn create_pokemon_form_type(
    pool: &State<PgPool>,
    data: Json<CreatePokemonFormType>,
) -> ApiResult<Json<PokemonFormType>> {
    let result = PokemonFormTypeService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/form-games/<pokemon_form_game_id>/types/<type_id>")]
pub async fn get_pokemon_form_type(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
    type_id: i32,
) -> ApiResult<Json<PokemonFormType>> {
    let result = PokemonFormTypeService::get(pool.inner(), pokemon_form_game_id, type_id).await?;
    Ok(Json(result))
}

#[get("/form-types")]
pub async fn list_pokemon_form_types(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokemonFormType>>> {
    let results = PokemonFormTypeService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/form-games/<pokemon_form_game_id>/types")]
pub async fn list_types_by_form_game(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
) -> ApiResult<Json<Vec<PokemonFormType>>> {
    let results = PokemonFormTypeService::list_by_form_game(pool.inner(), pokemon_form_game_id).await?;
    Ok(Json(results))
}

#[patch("/form-games/<pokemon_form_game_id>/types/<type_id>", format = "json", data = "<data>")]
pub async fn update_pokemon_form_type(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
    type_id: i32,
    data: Json<UpdatePokemonFormType>,
) -> ApiResult<Json<PokemonFormType>> {
    PokemonFormTypeService::update(pool.inner(), pokemon_form_game_id, type_id, data.into_inner()).await?;
    let result = PokemonFormTypeService::get(pool.inner(), pokemon_form_game_id, type_id).await?;
    Ok(Json(result))
}

#[delete("/form-games/<pokemon_form_game_id>/types/<type_id>")]
pub async fn delete_pokemon_form_type(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
    type_id: i32,
) -> ApiResult<Status> {
    PokemonFormTypeService::delete(pool.inner(), pokemon_form_game_id, type_id).await?;
    Ok(Status::NoContent)
}

pub fn pokemon_form_type_routes() -> Vec<Route> {
    routes![
        create_pokemon_form_type,
        get_pokemon_form_type,
        list_pokemon_form_types,
        list_types_by_form_game,
        update_pokemon_form_type,
        delete_pokemon_form_type
    ]
}
