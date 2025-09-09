use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokedexNumber {
    pub national_id: u16,
    pub pokedex_id: u16,
    pub pokedex_number: u16,
}
