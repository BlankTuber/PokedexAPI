use crate::error::ApiResult;
use crate::models::encounter_condition::CreateEncounterCondition;
use crate::validators::common::CommonValidator;

pub struct EncounterConditionValidator;

impl EncounterConditionValidator {
    pub fn validate_create(data: &CreateEncounterCondition) -> ApiResult<()> {
        CommonValidator::validate_positive(data.encounter_id, "Encounter ID")?;
        CommonValidator::validate_positive(data.encounter_condition_value_id, "Encounter condition value ID")?;
        Ok(())
    }
}
