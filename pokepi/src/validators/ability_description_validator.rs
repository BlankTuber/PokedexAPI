use crate::error::ApiResult;
use crate::models::ability_description::{CreateAbilityDescription, UpdateAbilityDescription};
use crate::validators::common::CommonValidator;

pub struct AbilityDescriptionValidator;

impl AbilityDescriptionValidator {
    pub fn validate_create(data: &CreateAbilityDescription) -> ApiResult<()> {
        CommonValidator::validate_positive(data.ability_id, "Ability ID")?;
        CommonValidator::validate_positive(data.version_group_id, "Version group ID")?;
        CommonValidator::validate_non_empty(&data.flavor_text, "Flavor text")?;
        CommonValidator::validate_non_empty(&data.short_effect, "Short effect")?;
        CommonValidator::validate_non_empty(&data.effect, "Effect")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateAbilityDescription) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.flavor_text, "Flavor text")?;
        CommonValidator::validate_optional_non_empty(&data.short_effect, "Short effect")?;
        CommonValidator::validate_optional_non_empty(&data.effect, "Effect")?;
        Ok(())
    }
}
