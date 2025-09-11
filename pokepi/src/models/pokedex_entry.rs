use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokedexEntry {
    pub national_id: u16,
    pub form_id: Option<u16>,
    pub game_id: u16,
    pub pokedex_number: u16,
    pub entry_text: String,
}

pub type CreatePokedexEntry = PokedexEntry;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokedexEntry {
    pub pokedex_number: Option<u16>,
    pub entry_text: Option<String>,
}
