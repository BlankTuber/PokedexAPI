use crate::error::ApiResult;
use crate::models::pokemon_form::{CreatePokemonForm, PokemonForm, UpdatePokemonForm};
use crate::services::pokemon_form_service::PokemonFormService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/forms", format = "json", data = "<data>")]
pub async fn create_pokemon_form(
    pool: &State<PgPool>,
    data: Json<CreatePokemonForm>,
) -> ApiResult<Json<PokemonForm>> {
    let form = PokemonFormService::create_pokemon_form(pool.inner(), data.into_inner()).await?;
    Ok(Json(form))
}

#[get("/forms/<form_id>")]
pub async fn get_pokemon_form(pool: &State<PgPool>, form_id: i32) -> ApiResult<Json<PokemonForm>> {
    let form = PokemonFormService::get_pokemon_form(pool.inner(), form_id).await?;
    Ok(Json(form))
}

#[get("/forms")]
pub async fn list_pokemon_forms(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokemonForm>>> {
    let forms = PokemonFormService::list_pokemon_forms(pool.inner()).await?;
    Ok(Json(forms))
}

#[get("/pokemon/<national_id>/forms")]
pub async fn list_forms_by_pokemon(pool: &State<PgPool>, national_id: i32) -> ApiResult<Json<Vec<PokemonForm>>> {
    let forms = PokemonFormService::list_forms_by_pokemon(pool.inner(), national_id).await?;
    Ok(Json(forms))
}

#[patch("/forms/<form_id>", format = "json", data = "<data>")]
pub async fn update_pokemon_form(
    pool: &State<PgPool>,
    form_id: i32,
    data: Json<UpdatePokemonForm>,
) -> ApiResult<Json<PokemonForm>> {
    PokemonFormService::update_pokemon_form(pool.inner(), form_id, data.into_inner()).await?;
    let form = PokemonFormService::get_pokemon_form(pool.inner(), form_id).await?;
    Ok(Json(form))
}

#[delete("/forms/<form_id>")]
pub async fn delete_pokemon_form(pool: &State<PgPool>, form_id: i32) -> ApiResult<Status> {
    PokemonFormService::delete_pokemon_form(pool.inner(), form_id).await?;
    Ok(Status::NoContent)
}

pub fn pokemon_form_routes() -> Vec<Route> {
    routes![
        create_pokemon_form,
        get_pokemon_form,
        list_pokemon_forms,
        list_forms_by_pokemon,
        update_pokemon_form,
        delete_pokemon_form
    ]
}
