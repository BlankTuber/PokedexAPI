use rocket::{Route, State, http::Status, serde::json::Json};
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::evolution_chain::{CreateEvolutionChain, EvolutionChain, UpdateEvolutionChain},
    services::evolution_chain_service::EvolutionChainService,
};

#[post("/evolution-chains", format = "json", data = "<data>")]
pub async fn create_evolution_chain(
    pool: &State<PgPool>,
    data: Json<CreateEvolutionChain>,
) -> ApiResult<Json<EvolutionChain>> {
    let evolution_chain =
        EvolutionChainService::create_evolution_chain(pool.inner(), data.into_inner()).await?;
    Ok(Json(evolution_chain))
}

#[get("/evolution-chains/<id>")]
pub async fn get_evolution_chain(pool: &State<PgPool>, id: i32) -> ApiResult<Json<EvolutionChain>> {
    let evolution_chain = EvolutionChainService::get_evolution_chain(pool.inner(), id).await?;
    Ok(Json(evolution_chain))
}

#[get("/evolution-chains")]
pub async fn list_evolution_chains(pool: &State<PgPool>) -> ApiResult<Json<Vec<EvolutionChain>>> {
    let evolution_chains = EvolutionChainService::list_evolution_chains(pool.inner()).await?;
    Ok(Json(evolution_chains))
}

#[patch("/evolution-chains/<id>", format = "json", data = "<data>")]
pub async fn update_evolution_chain(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateEvolutionChain>,
) -> ApiResult<Json<EvolutionChain>> {
    EvolutionChainService::update_evolution_chain(pool.inner(), id, data.into_inner()).await?;
    let evolution_chain = EvolutionChainService::get_evolution_chain(pool.inner(), id).await?;
    Ok(Json(evolution_chain))
}

#[delete("/evolution-chains/<id>")]
pub async fn delete_evolution_chain(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    EvolutionChainService::delete_evolution_chain(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn evolution_chain_routes() -> Vec<Route> {
    routes![
        create_evolution_chain,
        get_evolution_chain,
        list_evolution_chains,
        update_evolution_chain,
        delete_evolution_chain
    ]
}
