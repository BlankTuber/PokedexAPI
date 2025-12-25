use crate::error::ApiResult;
use crate::models::pokemon_form_type::{CreatePokemonFormType, UpdatePokemonFormType};
use crate::validators::common::CommonValidator;

pub struct PokemonFormTypeValidator;

impl PokemonFormTypeValidator {
    pub fn validate_create(data: &CreatePokemonFormType) -> ApiResult<()> {
        CommonValidator::validate_positive(data.pokemon_form_game_id, "Pokemon form game ID")?;
        CommonValidator::validate_positive(data.type_id, "Type ID")?;
        CommonValidator::validate_range(data.slot, 1, 2, "Slot")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokemonFormType) -> ApiResult<()> {
        if let Some(slot) = data.slot {
            CommonValidator::validate_range(slot, 1, 2, "Slot")?;
        }
        Ok(())
    }
}
