use crate::{
    error::ApiResult,
    models::platform::{CreatePlatform, UpdatePlatform},
    validators::common::CommonValidator,
};

pub struct PlatformValidator;

impl PlatformValidator {
    pub fn validate_create(data: &CreatePlatform) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.platform_name, "Platform name")?;
        CommonValidator::validate_identifier(&data.platform_identifier, "Platform identifier")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePlatform) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.platform_name, "Platform name")?;
        CommonValidator::validate_optional_identifier(
            &data.platform_identifier,
            "Platform identifier",
        )?;
        Ok(())
    }
}
