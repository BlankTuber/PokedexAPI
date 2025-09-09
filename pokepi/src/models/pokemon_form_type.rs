use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonFormType {
    pub pokemon_form_game_id: u32,
    pub type_id: u8,
    pub slot: u8,
}
