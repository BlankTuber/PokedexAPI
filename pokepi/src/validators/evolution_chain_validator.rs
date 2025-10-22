use crate::{
    error::ApiResult,
    models::evolution_chain::{CreateEvolutionChain, UpdateEvolutionChain},
    validators::common::CommonValidator,
};

pub struct EvolutionChainValidator;

impl EvolutionChainValidator {
    pub fn validate_create(data: &CreateEvolutionChain) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.evolution_chain_name, "Evolution chain name")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateEvolutionChain) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(
            &data.evolution_chain_name,
            "Evolution chain name",
        )?;
        Ok(())
    }
}
