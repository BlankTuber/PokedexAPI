use crate::error::ApiResult;
use crate::models::type_matchup::{CreateTypeMatchup, UpdateTypeMatchup};
use crate::validators::common::CommonValidator;

pub struct TypeMatchupValidator;

impl TypeMatchupValidator {
    pub fn validate_create(data: &CreateTypeMatchup) -> ApiResult<()> {
        CommonValidator::validate_positive(data.attacking_type_id, "Attacking type ID")?;
        CommonValidator::validate_positive(data.defending_type_id, "Defending type ID")?;
        // damage_factor is typically 0, 0.5, 1, or 2
        if data.damage_factor < 0.0 || data.damage_factor > 4.0 {
            return Err(crate::error::ApiError::BadRequest(
                "Damage factor must be between 0 and 4".to_string()
            ));
        }
        Ok(())
    }

    pub fn validate_update(data: &UpdateTypeMatchup) -> ApiResult<()> {
        if let Some(factor) = data.damage_factor {
            if factor < 0.0 || factor > 4.0 {
                return Err(crate::error::ApiError::BadRequest(
                    "Damage factor must be between 0 and 4".to_string()
                ));
            }
        }
        Ok(())
    }
}
