use crate::error::ApiResult;
use crate::models::encounter::{CreateEncounter, UpdateEncounter};
use crate::validators::common::CommonValidator;

pub struct EncounterValidator;

impl EncounterValidator {
    pub fn validate_create(data: &CreateEncounter) -> ApiResult<()> {
        CommonValidator::validate_positive(data.national_id, "National ID")?;
        CommonValidator::validate_positive(data.form_id, "Form ID")?;
        CommonValidator::validate_positive(data.game_id, "Game ID")?;
        CommonValidator::validate_positive(data.location_area_id, "Location area ID")?;
        CommonValidator::validate_positive(data.encounter_method_id, "Encounter method ID")?;
        CommonValidator::validate_range(data.chance, 0, 100, "Chance")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateEncounter) -> ApiResult<()> {
        if let Some(id) = data.national_id {
            CommonValidator::validate_positive(id, "National ID")?;
        }
        if let Some(id) = data.form_id {
            CommonValidator::validate_positive(id, "Form ID")?;
        }
        if let Some(id) = data.game_id {
            CommonValidator::validate_positive(id, "Game ID")?;
        }
        if let Some(id) = data.location_area_id {
            CommonValidator::validate_positive(id, "Location area ID")?;
        }
        if let Some(id) = data.encounter_method_id {
            CommonValidator::validate_positive(id, "Encounter method ID")?;
        }
        if let Some(chance) = data.chance {
            CommonValidator::validate_range(chance, 0, 100, "Chance")?;
        }
        Ok(())
    }
}
