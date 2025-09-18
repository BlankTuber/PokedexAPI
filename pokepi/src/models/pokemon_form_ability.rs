use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonFormAbility {
    pub pokemon_form_game_id: i32,
    pub ability_id: i32,
    pub slot: i16,
    pub is_hidden: bool,
}

pub type CreatePokemonFormAbility = PokemonFormAbility;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonFormAbility {
    pub slot: Option<i16>,
    pub is_hidden: Option<bool>,
}
