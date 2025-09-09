use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonType {
    pub type_id: u8,
    pub type_name: String,
    pub type_identifier: String,
    pub generation_introduced: u8,
}

pub type CreatePokemonType = PokemonType;

// Route: PATCH /types/<type_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemonType {
    pub type_name: Option<String>,
    pub type_identifier: Option<String>,
    pub generation_introduced: Option<u8>,
}
