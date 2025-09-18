use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokedexNumber {
    pub national_id: i32,
    pub pokedex_id: i32,
    pub pokedex_number: i32,
}

pub type CreatePokedexNumber = PokedexNumber;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokedexNumber {
    pub pokedex_number: Option<i32>,
}
