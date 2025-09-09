use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RegionalPokedex {
    pub pokedex_id: u16,
    pub pokedex_name: String,
    pub pokedex_identifier: String,
    pub region_id: u16,
    pub version_group_id: u16,
    pub is_main_series: bool,
}
