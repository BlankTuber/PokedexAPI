use crate::error::ApiResult;
use crate::models::region::{CreateRegion, UpdateRegion};
use crate::validators::common::CommonValidator;

pub struct RegionValidator;

impl RegionValidator {
    pub fn validate_create(data: &CreateRegion) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.region_name, "Region name")?;
        CommonValidator::validate_identifier(&data.region_identifier, "Region identifier")?;
        CommonValidator::validate_range(data.generation_introduced, 1, 15, "Generation introduced")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateRegion) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.region_name, "Region name")?;
        CommonValidator::validate_optional_identifier(&data.region_identifier, "Region identifier")?;
        if let Some(generation) = data.generation_introduced {
            CommonValidator::validate_range(generation, 1, 15, "Generation introduced")?;
        }
        Ok(())
    }
}
