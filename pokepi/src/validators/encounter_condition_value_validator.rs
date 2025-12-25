use crate::error::ApiResult;
use crate::models::encounter_condition_values::{CreateEncounterConditionValue, UpdateEncounterConditionValue};
use crate::validators::common::CommonValidator;

pub struct EncounterConditionValueValidator;

impl EncounterConditionValueValidator {
    pub fn validate_create(data: &CreateEncounterConditionValue) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.value_name, "Value name")?;
        CommonValidator::validate_identifier(&data.value_identifier, "Value identifier")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateEncounterConditionValue) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.value_name, "Value name")?;
        CommonValidator::validate_optional_identifier(&data.value_identifier, "Value identifier")?;
        Ok(())
    }
}
