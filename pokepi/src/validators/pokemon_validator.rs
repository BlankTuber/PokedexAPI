use crate::error::ApiResult;
use crate::models::pokemon::{CreatePokemon, UpdatePokemon};
use crate::validators::common::CommonValidator;

pub struct PokemonValidator;

impl PokemonValidator {
    pub fn validate_create(data: &CreatePokemon) -> ApiResult<()> {
        CommonValidator::validate_positive(data.national_id, "National ID")?;
        CommonValidator::validate_non_empty(&data.species_name, "Species name")?;
        CommonValidator::validate_non_empty(&data.classification, "Classification")?;
        CommonValidator::validate_range(data.gender_ratio, -1.0, 100.0, "Gender ratio")?;
        CommonValidator::validate_positive(data.evolution_chain_id, "Evolution chain ID")?;
        CommonValidator::validate_range(data.generation_introduced, 1, 15, "Generation introduced")?;
        CommonValidator::validate_range(data.capture_rate, 0, 255, "Capture rate")?;
        CommonValidator::validate_range(data.base_happiness, 0, 255, "Base happiness")?;
        CommonValidator::validate_positive(data.growth_rate_id, "Growth rate ID")?;
        CommonValidator::validate_positive(data.egg_group_1_id, "Egg group 1 ID")?;
        CommonValidator::validate_range(data.hatch_cycles, 1, 255, "Hatch cycles")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokemon) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.species_name, "Species name")?;
        CommonValidator::validate_optional_non_empty(&data.classification, "Classification")?;
        if let Some(ratio) = data.gender_ratio {
            CommonValidator::validate_range(ratio, -1.0, 100.0, "Gender ratio")?;
        }
        if let Some(generation) = data.generation_introduced {
            CommonValidator::validate_range(generation, 1, 15, "Generation introduced")?;
        }
        if let Some(rate) = data.capture_rate {
            CommonValidator::validate_range(rate, 0, 255, "Capture rate")?;
        }
        if let Some(happiness) = data.base_happiness {
            CommonValidator::validate_range(happiness, 0, 255, "Base happiness")?;
        }
        if let Some(cycles) = data.hatch_cycles {
            CommonValidator::validate_range(cycles, 1, 255, "Hatch cycles")?;
        }
        Ok(())
    }
}
