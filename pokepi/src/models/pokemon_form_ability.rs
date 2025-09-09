use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonFormAbility {
    pub pokemon_form_game_id: u32,
    pub ability_id: u16,
    pub slot: u8,
    pub is_hidden: bool,
}
