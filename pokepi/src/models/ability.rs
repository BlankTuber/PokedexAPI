use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ability {
    pub ability_id: i16,
    pub ability_name: String,
    pub ability_identifier: String,
    pub generation_introduced: u8,
    pub is_main_series: bool,
}
