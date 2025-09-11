use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonFormType {
    pub pokemon_form_game_id: u32,
    pub type_id: u8,
    pub slot: u8,
}

pub type CreatePokemonFormType = PokemonFormType;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonFormType {
    pub slot: Option<u8>,
}
