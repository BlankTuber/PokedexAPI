use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonFormGame {
    pub pokemon_form_game_id: u32,
    pub national_id: u16,
    pub form_id: u16,
    pub game_id: u16,
    pub is_available: bool,
    pub is_shiny_locked: bool,
    pub height: f32,
    pub weight: f32,
    pub base_experience: u16,
    pub hp: Option<u8>,
    pub attack: Option<u8>,
    pub defense: Option<u8>,
    pub special_attack: Option<u8>,
    pub special_defense: Option<u8>,
    pub speed: Option<u8>,
}
