use crate::error::ApiResult;
use crate::models::regional_pokedex::{CreateRegionalPokedex, UpdateRegionalPokedex};
use crate::validators::common::CommonValidator;

pub struct RegionalPokedexValidator;

impl RegionalPokedexValidator {
    pub fn validate_create(data: &CreateRegionalPokedex) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.pokedex_name, "Pokedex name")?;
        CommonValidator::validate_identifier(&data.pokedex_identifier, "Pokedex identifier")?;
        CommonValidator::validate_positive(data.region_id, "Region ID")?;
        CommonValidator::validate_positive(data.version_group_id, "Version group ID")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateRegionalPokedex) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.pokedex_name, "Pokedex name")?;
        CommonValidator::validate_optional_identifier(&data.pokedex_identifier, "Pokedex identifier")?;
        Ok(())
    }
}
