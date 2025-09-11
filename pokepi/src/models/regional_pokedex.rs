use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionalPokedex {
    pub pokedex_id: u16,
    pub pokedex_name: String,
    pub pokedex_identifier: String,
    pub region_id: u16,
    pub version_group_id: u16,
    pub is_main_series: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateRegionalPokedex {
    pub pokedex_name: String,
    pub pokedex_identifier: String,
    pub region_id: u16,
    pub version_group_id: u16,
    pub is_main_series: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateRegionalPokedex {
    pub pokedex_name: Option<String>,
    pub pokedex_identifier: Option<String>,
    pub is_main_series: Option<bool>,
}
