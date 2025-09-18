use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonFormType {
    pub pokemon_form_game_id: i32,
    pub type_id: i32,
    pub slot: i16,
}

pub type CreatePokemonFormType = PokemonFormType;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonFormType {
    pub slot: Option<i16>,
}
