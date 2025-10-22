use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::evolution_chain::{CreateEvolutionChain, EvolutionChain, UpdateEvolutionChain},
    repositories::evolution_chain_repository::EvolutionChainRepository,
    validators::evolution_chain_validator::EvolutionChainValidator,
};

pub struct EvolutionChainService;

impl EvolutionChainService {
    pub async fn create_evolution_chain(
        pool: &PgPool,
        data: CreateEvolutionChain,
    ) -> ApiResult<EvolutionChain> {
        EvolutionChainValidator::validate_create(&data)?;
        EvolutionChainRepository::create(pool, data).await
    }

    pub async fn get_evolution_chain(pool: &PgPool, id: i32) -> ApiResult<EvolutionChain> {
        EvolutionChainRepository::find_by_id(pool, id).await
    }

    pub async fn list_evolution_chains(pool: &PgPool) -> ApiResult<Vec<EvolutionChain>> {
        EvolutionChainRepository::find_all(pool).await
    }

    pub async fn update_evolution_chain(
        pool: &PgPool,
        id: i32,
        data: UpdateEvolutionChain,
    ) -> ApiResult<()> {
        EvolutionChainValidator::validate_update(&data)?;
        EvolutionChainRepository::update(pool, id, data).await
    }

    pub async fn delete_evolution_chain(pool: &PgPool, id: i32) -> ApiResult<()> {
        EvolutionChainRepository::delete(pool, id).await
    }
}
