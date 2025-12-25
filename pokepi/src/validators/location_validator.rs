use crate::error::ApiResult;
use crate::models::location::{CreateLocation, UpdateLocation};
use crate::validators::common::CommonValidator;

pub struct LocationValidator;

impl LocationValidator {
    pub fn validate_create(data: &CreateLocation) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.location_name, "Location name")?;
        CommonValidator::validate_identifier(&data.location_identifier, "Location identifier")?;
        CommonValidator::validate_positive(data.region_id, "Region ID")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateLocation) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.location_name, "Location name")?;
        CommonValidator::validate_optional_identifier(&data.location_identifier, "Location identifier")?;
        Ok(())
    }
}
