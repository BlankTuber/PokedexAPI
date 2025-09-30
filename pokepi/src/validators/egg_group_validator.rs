use crate::error::ApiResult;
use crate::models::egg_group::{CreateEggGroup, UpdateEggGroup};
use crate::validators::common::CommonValidator;

pub struct EggGroupValidator;

impl EggGroupValidator {
    pub fn validate_create(data: &CreateEggGroup) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.egg_group_name, "Egg group name")?;
        CommonValidator::validate_identifier(&data.egg_group_identifier, "Egg group identifier")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateEggGroup) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.egg_group_name, "Egg group name")?;
        CommonValidator::validate_optional_identifier(
            &data.egg_group_identifier,
            "Egg group identifier",
        )?;
        Ok(())
    }
}
