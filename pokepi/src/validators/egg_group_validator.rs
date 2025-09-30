use crate::error::{ApiError, ApiResult};
use crate::models::egg_group::{CreateEggGroup, UpdateEggGroup};

pub struct EggGroupValidator;

impl EggGroupValidator {
    pub fn validate_create(data: &CreateEggGroup) -> ApiResult<()> {
        if data.egg_group_name.trim().is_empty() {
            return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
        }

        if data.egg_group_identifier.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Identifier cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    pub fn validate_update(data: &UpdateEggGroup) -> ApiResult<()> {
        if let Some(name) = &data.egg_group_name {
            if name.trim().is_empty() {
                return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
            }
        }

        if let Some(identifier) = &data.egg_group_identifier {
            if identifier.trim().is_empty() {
                return Err(ApiError::BadRequest(
                    "Identifier cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}
