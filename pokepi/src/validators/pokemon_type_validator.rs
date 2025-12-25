use crate::error::ApiResult;
use crate::models::pokemon_type::{CreatePokemonType, UpdatePokemonType};
use crate::validators::common::CommonValidator;

pub struct PokemonTypeValidator;

impl PokemonTypeValidator {
    pub fn validate_create(data: &CreatePokemonType) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.type_name, "Type name")?;
        CommonValidator::validate_identifier(&data.type_identifier, "Type identifier")?;
        CommonValidator::validate_range(data.generation_introduced, 1, 15, "Generation introduced")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokemonType) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.type_name, "Type name")?;
        CommonValidator::validate_optional_identifier(&data.type_identifier, "Type identifier")?;
        if let Some(generation) = data.generation_introduced {
            CommonValidator::validate_range(generation, 1, 15, "Generation introduced")?;
        }
        Ok(())
    }
}
