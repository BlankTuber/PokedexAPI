use crate::error::ApiResult;
use crate::models::regional_pokedex::{CreateRegionalPokedex, RegionalPokedex, UpdateRegionalPokedex};
use crate::services::regional_pokedex_service::RegionalPokedexService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/regional-pokedexes", format = "json", data = "<data>")]
pub async fn create_regional_pokedex(
    pool: &State<PgPool>,
    data: Json<CreateRegionalPokedex>,
) -> ApiResult<Json<RegionalPokedex>> {
    let result = RegionalPokedexService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/regional-pokedexes/<id>")]
pub async fn get_regional_pokedex(pool: &State<PgPool>, id: i32) -> ApiResult<Json<RegionalPokedex>> {
    let result = RegionalPokedexService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/regional-pokedexes")]
pub async fn list_regional_pokedexes(pool: &State<PgPool>) -> ApiResult<Json<Vec<RegionalPokedex>>> {
    let results = RegionalPokedexService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[get("/regions/<region_id>/pokedexes")]
pub async fn list_pokedexes_by_region(
    pool: &State<PgPool>,
    region_id: i32,
) -> ApiResult<Json<Vec<RegionalPokedex>>> {
    let results = RegionalPokedexService::list_by_region(pool.inner(), region_id).await?;
    Ok(Json(results))
}

#[patch("/regional-pokedexes/<id>", format = "json", data = "<data>")]
pub async fn update_regional_pokedex(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateRegionalPokedex>,
) -> ApiResult<Json<RegionalPokedex>> {
    RegionalPokedexService::update(pool.inner(), id, data.into_inner()).await?;
    let result = RegionalPokedexService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/regional-pokedexes/<id>")]
pub async fn delete_regional_pokedex(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    RegionalPokedexService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn regional_pokedex_routes() -> Vec<Route> {
    routes![
        create_regional_pokedex,
        get_regional_pokedex,
        list_regional_pokedexes,
        list_pokedexes_by_region,
        update_regional_pokedex,
        delete_regional_pokedex
    ]
}
