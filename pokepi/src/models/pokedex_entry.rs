use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokedexEntry {
    pub national_id: i32,
    pub form_id: Option<i32>,
    pub game_id: i32,
    pub pokedex_number: i32,
    pub entry_text: String,
}

pub type CreatePokedexEntry = PokedexEntry;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokedexEntry {
    pub pokedex_number: Option<i32>,
    pub entry_text: Option<String>,
}
