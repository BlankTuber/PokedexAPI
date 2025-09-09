use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PokedexEntry {
    pub national_id: u16,
    pub form_id: Option<u16>,
    pub game_id: u16,
    pub pokedex_number: u16,
    pub entry_text: String,
}
