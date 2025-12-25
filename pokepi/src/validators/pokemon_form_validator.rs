use crate::error::ApiResult;
use crate::models::pokemon_form::{CreatePokemonForm, UpdatePokemonForm};
use crate::validators::common::CommonValidator;

pub struct PokemonFormValidator;

impl PokemonFormValidator {
    pub fn validate_create(data: &CreatePokemonForm) -> ApiResult<()> {
        CommonValidator::validate_positive(data.national_id, "National ID")?;
        CommonValidator::validate_non_empty(&data.form_name, "Form name")?;
        CommonValidator::validate_identifier(&data.form_identifier, "Form identifier")?;
        CommonValidator::validate_non_empty(&data.form_type, "Form type")?;
        CommonValidator::validate_non_empty(&data.sprite_name, "Sprite name")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokemonForm) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.form_name, "Form name")?;
        CommonValidator::validate_optional_identifier(&data.form_identifier, "Form identifier")?;
        CommonValidator::validate_optional_non_empty(&data.form_type, "Form type")?;
        CommonValidator::validate_optional_non_empty(&data.sprite_name, "Sprite name")?;
        Ok(())
    }
}
