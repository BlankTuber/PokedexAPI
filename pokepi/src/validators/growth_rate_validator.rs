use crate::{
    error::ApiResult,
    models::growth_rate::{CreateGrowthRate, UpdateGrowthRate},
    validators::common::CommonValidator,
};

pub struct GrowthRateValidator;

impl GrowthRateValidator {
    pub fn validate_create(data: &CreateGrowthRate) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.growth_rate_name, "Growth rate name")?;
        CommonValidator::validate_identifier(
            &data.growth_rate_identifier,
            "Growth rate identifier",
        )?;
        CommonValidator::validate_non_empty(&data.formula, "Growth rate formula")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateGrowthRate) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.growth_rate_name, "Growth rate name")?;
        CommonValidator::validate_optional_identifier(
            &data.growth_rate_identifier,
            "Growth rate identifier",
        )?;
        CommonValidator::validate_optional_non_empty(&data.formula, "Growth rate formula")?;
        Ok(())
    }
}
