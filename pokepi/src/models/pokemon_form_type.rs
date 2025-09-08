use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonFormType {
    pub pokemon_form_game_id: i16,
    pub type_id: i8,
    pub slot: i8,
}
