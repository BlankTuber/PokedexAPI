use crate::error::ApiResult;
use crate::models::game::{CreateGame, UpdateGame};
use crate::validators::common::CommonValidator;

pub struct GameValidator;

impl GameValidator {
    pub fn validate_create(data: &CreateGame) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.game_name, "Game name")?;
        CommonValidator::validate_identifier(&data.game_identifier, "Game identifier")?;
        CommonValidator::validate_range(data.generation, 1, 15, "Generation")?;
        CommonValidator::validate_positive(data.version_group_id, "Version group ID")?;
        CommonValidator::validate_positive(data.platform_id, "Platform ID")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateGame) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.game_name, "Game name")?;
        CommonValidator::validate_optional_identifier(&data.game_identifier, "Game identifier")?;
        if let Some(generation) = data.generation {
            CommonValidator::validate_range(generation, 1, 15, "Generation")?;
        }
        if let Some(vg_id) = data.version_group_id {
            CommonValidator::validate_positive(vg_id, "Version group ID")?;
        }
        if let Some(p_id) = data.platform_id {
            CommonValidator::validate_positive(p_id, "Platform ID")?;
        }
        Ok(())
    }
}
