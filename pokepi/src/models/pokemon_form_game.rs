use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonFormGame {
    pub pokemon_form_game_id: i32,
    pub national_id: i32,
    pub form_id: i32,
    pub game_id: i32,
    pub is_available: bool,
    pub is_shiny_locked: bool,
    pub height: f32,
    pub weight: f32,
    pub base_experience: i32,
    pub hp: Option<i16>,
    pub attack: Option<i16>,
    pub defense: Option<i16>,
    pub special_attack: Option<i16>,
    pub special_defense: Option<i16>,
    pub speed: Option<i16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatePokemonFormGame {
    pub national_id: i32,
    pub form_id: i32,
    pub game_id: i32,
    pub is_available: bool,
    pub is_shiny_locked: bool,
    pub height: f32,
    pub weight: f32,
    pub base_experience: i32,
    pub hp: Option<i16>,
    pub attack: Option<i16>,
    pub defense: Option<i16>,
    pub special_attack: Option<i16>,
    pub special_defense: Option<i16>,
    pub speed: Option<i16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonFormGame {
    pub is_available: Option<bool>,
    pub is_shiny_locked: Option<bool>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub base_experience: Option<i32>,
    pub hp: Option<Option<i16>>,
    pub attack: Option<Option<i16>>,
    pub defense: Option<Option<i16>>,
    pub special_attack: Option<Option<i16>>,
    pub special_defense: Option<Option<i16>>,
    pub speed: Option<Option<i16>>,
}
