use crate::{
    error::ApiResult,
    models::version_group::{CreateVersionGroup, UpdateVersionGroup},
    validators::common::CommonValidator,
};

pub struct VersionGroupValidator;

impl VersionGroupValidator {
    pub fn validate_create(data: &CreateVersionGroup) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.version_group_name, "Version group name")?;
        CommonValidator::validate_identifier(
            &data.version_group_identifier,
            "Version group identifier",
        )?;
        CommonValidator::validate_range(data.generation, 1, 9, "Generation")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateVersionGroup) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(
            &data.version_group_name,
            "Version group name",
        )?;
        CommonValidator::validate_optional_identifier(
            &data.version_group_identifier,
            "Version group identifier",
        )?;
        CommonValidator::validate_optional_range(&data.generation, 1, 9, "Generation")?;
        Ok(())
    }
}
