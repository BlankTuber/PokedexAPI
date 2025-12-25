use crate::error::ApiResult;
use crate::models::pokemon_form_ability::{CreatePokemonFormAbility, UpdatePokemonFormAbility};
use crate::validators::common::CommonValidator;

pub struct PokemonFormAbilityValidator;

impl PokemonFormAbilityValidator {
    pub fn validate_create(data: &CreatePokemonFormAbility) -> ApiResult<()> {
        CommonValidator::validate_positive(data.pokemon_form_game_id, "Pokemon form game ID")?;
        CommonValidator::validate_positive(data.ability_id, "Ability ID")?;
        CommonValidator::validate_range(data.slot, 1, 3, "Slot")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokemonFormAbility) -> ApiResult<()> {
        if let Some(slot) = data.slot {
            CommonValidator::validate_range(slot, 1, 3, "Slot")?;
        }
        Ok(())
    }
}
