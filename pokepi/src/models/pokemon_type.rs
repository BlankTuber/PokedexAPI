use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokemonType {
    pub type_id: i8,
    pub type_name: String,
    pub type_identifier: String,
    pub generation_introduced: u8,
}
