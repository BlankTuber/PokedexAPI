use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonForm {
    pub form_id: u16,
    pub national_id: u16,
    pub form_name: String,
    pub form_identifier: String,
    pub form_type: String,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub sprite_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatePokemonForm {
    pub national_id: u16,
    pub form_name: String,
    pub form_identifier: String,
    pub form_type: String,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub sprite_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonForm {
    pub form_name: Option<String>,
    pub form_identifier: Option<String>,
    pub form_type: Option<String>,
    pub is_default: Option<bool>,
    pub is_battle_only: Option<bool>,
    pub sprite_name: Option<String>,
}
