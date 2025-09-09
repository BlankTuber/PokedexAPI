use serde::Serialize;

#[derive(Debug, Serialize)]
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
