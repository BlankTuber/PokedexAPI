use crate::error::ApiResult;
use crate::models::location_area::{CreateLocationArea, UpdateLocationArea};
use crate::validators::common::CommonValidator;

pub struct LocationAreaValidator;

impl LocationAreaValidator {
    pub fn validate_create(data: &CreateLocationArea) -> ApiResult<()> {
        CommonValidator::validate_positive(data.location_id, "Location ID")?;
        CommonValidator::validate_non_empty(&data.area_name, "Area name")?;
        CommonValidator::validate_identifier(&data.area_identifier, "Area identifier")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateLocationArea) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.area_name, "Area name")?;
        CommonValidator::validate_optional_identifier(&data.area_identifier, "Area identifier")?;
        Ok(())
    }
}
