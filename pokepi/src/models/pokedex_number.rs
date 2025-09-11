use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokedexNumber {
    pub national_id: u16,
    pub pokedex_id: u16,
    pub pokedex_number: u16,
}

pub type CreatePokedexNumber = PokedexNumber;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokedexNumber {
    pub pokedex_number: Option<u16>,
}
