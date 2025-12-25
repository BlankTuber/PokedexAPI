use crate::error::ApiResult;
use crate::models::pokemon_form_game::{CreatePokemonFormGame, UpdatePokemonFormGame};
use crate::validators::common::CommonValidator;

pub struct PokemonFormGameValidator;

impl PokemonFormGameValidator {
    pub fn validate_create(data: &CreatePokemonFormGame) -> ApiResult<()> {
        CommonValidator::validate_positive(data.national_id, "National ID")?;
        CommonValidator::validate_positive(data.form_id, "Form ID")?;
        CommonValidator::validate_positive(data.game_id, "Game ID")?;
        CommonValidator::validate_positive(data.base_experience, "Base experience")?;
        if data.height <= 0.0 {
            return Err(crate::error::ApiError::BadRequest("Height must be positive".to_string()));
        }
        if data.weight <= 0.0 {
            return Err(crate::error::ApiError::BadRequest("Weight must be positive".to_string()));
        }
        // Validate stats if provided
        if let Some(hp) = data.hp {
            CommonValidator::validate_range(hp, 1, 255, "HP")?;
        }
        if let Some(attack) = data.attack {
            CommonValidator::validate_range(attack, 1, 255, "Attack")?;
        }
        if let Some(defense) = data.defense {
            CommonValidator::validate_range(defense, 1, 255, "Defense")?;
        }
        if let Some(sp_atk) = data.special_attack {
            CommonValidator::validate_range(sp_atk, 1, 255, "Special Attack")?;
        }
        if let Some(sp_def) = data.special_defense {
            CommonValidator::validate_range(sp_def, 1, 255, "Special Defense")?;
        }
        if let Some(speed) = data.speed {
            CommonValidator::validate_range(speed, 1, 255, "Speed")?;
        }
        Ok(())
    }

    pub fn validate_update(data: &UpdatePokemonFormGame) -> ApiResult<()> {
        if let Some(height) = data.height {
            if height <= 0.0 {
                return Err(crate::error::ApiError::BadRequest("Height must be positive".to_string()));
            }
        }
        if let Some(weight) = data.weight {
            if weight <= 0.0 {
                return Err(crate::error::ApiError::BadRequest("Weight must be positive".to_string()));
            }
        }
        if let Some(base_exp) = data.base_experience {
            CommonValidator::validate_positive(base_exp, "Base experience")?;
        }
        // Validate nested optional stats
        if let Some(Some(hp)) = data.hp {
            CommonValidator::validate_range(hp, 1, 255, "HP")?;
        }
        if let Some(Some(attack)) = data.attack {
            CommonValidator::validate_range(attack, 1, 255, "Attack")?;
        }
        if let Some(Some(defense)) = data.defense {
            CommonValidator::validate_range(defense, 1, 255, "Defense")?;
        }
        if let Some(Some(sp_atk)) = data.special_attack {
            CommonValidator::validate_range(sp_atk, 1, 255, "Special Attack")?;
        }
        if let Some(Some(sp_def)) = data.special_defense {
            CommonValidator::validate_range(sp_def, 1, 255, "Special Defense")?;
        }
        if let Some(Some(speed)) = data.speed {
            CommonValidator::validate_range(speed, 1, 255, "Speed")?;
        }
        Ok(())
    }
}
