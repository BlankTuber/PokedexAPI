use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonFormAbility {
    pub pokemon_form_game_id: u32,
    pub ability_id: u16,
    pub slot: u8,
    pub is_hidden: bool,
}

pub type CreatePokemonFormAbility = PokemonFormAbility;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonFormAbility {
    pub slot: Option<u8>,
    pub is_hidden: Option<bool>,
}
