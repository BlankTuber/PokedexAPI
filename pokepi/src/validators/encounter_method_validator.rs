use crate::error::ApiResult;
use crate::models::encounter_method::{CreateEncounterMethod, UpdateEncounterMethod};
use crate::validators::common::CommonValidator;

pub struct EncounterMethodValidator;

impl EncounterMethodValidator {
    pub fn validate_create(data: &CreateEncounterMethod) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.method_name, "Method name")?;
        CommonValidator::validate_identifier(&data.method_identifier, "Method identifier")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateEncounterMethod) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.method_name, "Method name")?;
        CommonValidator::validate_optional_identifier(&data.method_identifier, "Method identifier")?;
        Ok(())
    }
}
