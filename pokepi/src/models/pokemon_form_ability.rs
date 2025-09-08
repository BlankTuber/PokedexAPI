use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonFormAbilitiy {
    pub pokemon_form_game_id: i16,
    pub ability_id: i16,
    pub slot: i8,
    pub is_hidden: bool,
}
