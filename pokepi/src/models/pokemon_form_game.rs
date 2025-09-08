use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonFormGame {
    pub pokemon_form_game_id: i16,
    pub national_id: i16,
    pub form_id: i16,
    pub game_id: i16,
    pub is_available: bool,
    pub is_shiny_locked: bool,
    pub height: f64,
    pub weight: f64,
    pub base_experience: i16,
    pub hp: Option<u8>,
    pub attack: Option<u8>,
    pub defense: Option<u8>,
    pub special_attack: Option<u8>,
    pub special_defense: Option<u8>,
    pub speed: Option<u8>,
}
