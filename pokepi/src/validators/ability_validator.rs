use crate::error::ApiResult;
use crate::models::ability::{CreateAbility, UpdateAbility};
use crate::validators::common::CommonValidator;

pub struct AbilityValidator;

impl AbilityValidator {
    pub fn validate_create(data: &CreateAbility) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.ability_name, "Ability name")?;
        CommonValidator::validate_identifier(&data.ability_identifier, "Ability identifier")?;
        CommonValidator::validate_range(data.generation_introduced, 1, 15, "Generation introduced")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateAbility) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.ability_name, "Ability name")?;
        CommonValidator::validate_optional_identifier(&data.ability_identifier, "Ability identifier")?;
        if let Some(generation) = data.generation_introduced {
            CommonValidator::validate_range(generation, 1, 15, "Generation introduced")?;
        }
        Ok(())
    }
}
