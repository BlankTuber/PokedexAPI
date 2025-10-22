use sqlx::PgPool;

use crate::{
    error::ApiResult,
    models::evolution_chain::{CreateEvolutionChain, EvolutionChain, UpdateEvolutionChain},
    validators::common::CommonValidator,
};

pub struct EvolutionChainRepository;

impl EvolutionChainRepository {
    pub async fn create(pool: &PgPool, data: CreateEvolutionChain) -> ApiResult<EvolutionChain> {
        let result = sqlx::query_as!(
            EvolutionChain,
            r#"
              INSERT INTO evolution_chains (evolution_chain_name)
              VALUES ($1)
              RETURNING evolution_chain_id, evolution_chain_name
            "#,
            data.evolution_chain_name
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<EvolutionChain> {
        let evolution_chain = sqlx::query_as!(
            EvolutionChain,
            r#"
              SELECT evolution_chain_id, evolution_chain_name
              FROM evolution_chains
              WHERE evolution_chain_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(evolution_chain)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<EvolutionChain>> {
        let evolution_chains = sqlx::query_as!(
            EvolutionChain,
            r#"
            SELECT evolution_chain_id, evolution_chain_name
            FROM evolution_chains
            ORDER BY evolution_chain_id
          "#
        )
        .fetch_all(pool)
        .await?;
        Ok(evolution_chains)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateEvolutionChain) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
              UPDATE evolution_chains
              SET evolution_chain_name = COALESCE($1, evolution_chain_name)
              WHERE evolution_chain_id = $2
            "#,
            data.evolution_chain_name,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Evolution chain")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM evolution_chains WHERE evolution_chain_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Evolution chain")?;

        Ok(())
    }
}
