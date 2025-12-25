use crate::error::ApiResult;
use crate::models::pokedex_number::{CreatePokedexNumber, PokedexNumber, UpdatePokedexNumber};
use crate::services::pokedex_number_service::PokedexNumberService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/pokedex-numbers", format = "json", data = "<data>")]
pub async fn create_pokedex_number(
    pool: &State<PgPool>,
    data: Json<CreatePokedexNumber>,
) -> ApiResult<Json<PokedexNumber>> {
    let result = PokedexNumberService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/pokemon/<national_id>/pokedex-numbers/<pokedex_id>")]
pub async fn get_pokedex_number(
    pool: &State<PgPool>,
    national_id: i32,
    pokedex_id: i32,
) -> ApiResult<Json<PokedexNumber>> {
    let result = PokedexNumberService::get(pool.inner(), national_id, pokedex_id).await?;
    Ok(Json(result))
}

#[get("/pokedex-numbers")]
pub async fn list_pokedex_numbers(pool: &State<PgPool>) -> ApiResult<Json<Vec<PokedexNumber>>> {
    let results = PokedexNumberService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/pokemon/<national_id>/pokedex-numbers")]
pub async fn list_numbers_by_pokemon(
    pool: &State<PgPool>,
    national_id: i32,
) -> ApiResult<Json<Vec<PokedexNumber>>> {
    let results = PokedexNumberService::list_by_pokemon(pool.inner(), national_id).await?;
    Ok(Json(results))
}

#[get("/regional-pokedexes/<pokedex_id>/numbers")]
pub async fn list_numbers_by_pokedex(
    pool: &State<PgPool>,
    pokedex_id: i32,
) -> ApiResult<Json<Vec<PokedexNumber>>> {
    let results = PokedexNumberService::list_by_pokedex(pool.inner(), pokedex_id).await?;
    Ok(Json(results))
}

#[patch("/pokemon/<national_id>/pokedex-numbers/<pokedex_id>", format = "json", data = "<data>")]
pub async fn update_pokedex_number(
    pool: &State<PgPool>,
    national_id: i32,
    pokedex_id: i32,
    data: Json<UpdatePokedexNumber>,
) -> ApiResult<Json<PokedexNumber>> {
    PokedexNumberService::update(pool.inner(), national_id, pokedex_id, data.into_inner()).await?;
    let result = PokedexNumberService::get(pool.inner(), national_id, pokedex_id).await?;
    Ok(Json(result))
}

#[delete("/pokemon/<national_id>/pokedex-numbers/<pokedex_id>")]
pub async fn delete_pokedex_number(
    pool: &State<PgPool>,
    national_id: i32,
    pokedex_id: i32,
) -> ApiResult<Status> {
    PokedexNumberService::delete(pool.inner(), national_id, pokedex_id).await?;
    Ok(Status::NoContent)
}

pub fn pokedex_number_routes() -> Vec<Route> {
    routes![
        create_pokedex_number,
        get_pokedex_number,
        list_pokedex_numbers,
        list_numbers_by_pokemon,
        list_numbers_by_pokedex,
        update_pokedex_number,
        delete_pokedex_number
    ]
}
