use crate::error::{ApiError, ApiResult};
use crate::models::pokedex_entry::{CreatePokedexEntry, UpdatePokedexEntry};
use crate::validators::common::CommonValidator;

pub struct PokedexEntryValidator;

impl PokedexEntryValidator {
    pub fn validate_create(data: &CreatePokedexEntry) -> ApiResult<()> {
        CommonValidator::validate_positive(data.national_id, "National ID")?;
        if let Some(form_id) = data.form_id {
            CommonValidator::validate_positive(form_id, "Form ID")?;
        }
        CommonValidator::validate_positive(data.game_id, "Game ID")?;
        CommonValidator::validate_positive(data.pokedex_number, "Pokedex number")?;
        CommonValidator::validate_non_empty(&data.entry_text, "Entry text")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokedexEntry) -> ApiResult<()> {
        if data.pokedex_number.is_none() && data.entry_text.is_none() {
            return Err(ApiError::BadRequest(
                "At least one field must be provided for update".to_string(),
            ));
        }
        if let Some(pokedex_number) = data.pokedex_number {
            CommonValidator::validate_positive(pokedex_number, "Pokedex number")?;
        }
        if let Some(ref entry_text) = data.entry_text {
            CommonValidator::validate_non_empty(entry_text, "Entry text")?;
        }
        Ok(())
    }
}
