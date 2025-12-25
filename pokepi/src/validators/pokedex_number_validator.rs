use crate::error::{ApiError, ApiResult};
use crate::models::pokedex_number::{CreatePokedexNumber, UpdatePokedexNumber};
use crate::validators::common::CommonValidator;

pub struct PokedexNumberValidator;

impl PokedexNumberValidator {
    pub fn validate_create(data: &CreatePokedexNumber) -> ApiResult<()> {
        CommonValidator::validate_positive(data.national_id, "National ID")?;
        CommonValidator::validate_positive(data.pokedex_id, "Pokedex ID")?;
        CommonValidator::validate_positive(data.pokedex_number, "Pokedex number")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokedexNumber) -> ApiResult<()> {
        if data.pokedex_number.is_none() {
            return Err(ApiError::BadRequest(
                "At least one field must be provided for update".to_string(),
            ));
        }
        if let Some(pokedex_number) = data.pokedex_number {
            CommonValidator::validate_positive(pokedex_number, "Pokedex number")?;
        }
        Ok(())
    }
}
