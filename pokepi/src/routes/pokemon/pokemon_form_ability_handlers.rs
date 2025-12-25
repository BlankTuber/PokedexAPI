use crate::error::ApiResult;
use crate::models::pokemon_form_ability::{CreatePokemonFormAbility, PokemonFormAbility, UpdatePokemonFormAbility};
use crate::services::pokemon_form_ability_service::PokemonFormAbilityService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/form-abilities", format = "json", data = "<data>")]
pub async fn create_pokemon_form_ability(
    pool: &State<PgPool>,
    data: Json<CreatePokemonFormAbility>,
) -> ApiResult<Json<PokemonFormAbility>> {
    let result = PokemonFormAbilityService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/form-games/<pokemon_form_game_id>/abilities/<ability_id>")]
pub async fn get_pokemon_form_ability(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
    ability_id: i32,
) -> ApiResult<Json<PokemonFormAbility>> {
    let result = PokemonFormAbilityService::get(pool.inner(), pokemon_form_game_id, ability_id).await?;
    Ok(Json(result))
}

#[get("/form-abilities")]
pub async fn list_pokemon_form_abilities(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokemonFormAbility>>> {
    let results = PokemonFormAbilityService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/form-games/<pokemon_form_game_id>/abilities")]
pub async fn list_abilities_by_form_game(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
) -> ApiResult<Json<Vec<PokemonFormAbility>>> {
    let results = PokemonFormAbilityService::list_by_form_game(pool.inner(), pokemon_form_game_id).await?;
    Ok(Json(results))
}

#[patch("/form-games/<pokemon_form_game_id>/abilities/<ability_id>", format = "json", data = "<data>")]
pub async fn update_pokemon_form_ability(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
    ability_id: i32,
    data: Json<UpdatePokemonFormAbility>,
) -> ApiResult<Json<PokemonFormAbility>> {
    PokemonFormAbilityService::update(pool.inner(), pokemon_form_game_id, ability_id, data.into_inner()).await?;
    let result = PokemonFormAbilityService::get(pool.inner(), pokemon_form_game_id, ability_id).await?;
    Ok(Json(result))
}

#[delete("/form-games/<pokemon_form_game_id>/abilities/<ability_id>")]
pub async fn delete_pokemon_form_ability(
    pool: &State<PgPool>,
    pokemon_form_game_id: i32,
    ability_id: i32,
) -> ApiResult<Status> {
    PokemonFormAbilityService::delete(pool.inner(), pokemon_form_game_id, ability_id).await?;
    Ok(Status::NoContent)
}

pub fn pokemon_form_ability_routes() -> Vec<Route> {
    routes![
        create_pokemon_form_ability,
        get_pokemon_form_ability,
        list_pokemon_form_abilities,
        list_abilities_by_form_game,
        update_pokemon_form_ability,
        delete_pokemon_form_ability
    ]
}
