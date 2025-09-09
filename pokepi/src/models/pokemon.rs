use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pokemon {
    pub national_id: u16,
    pub species_name: String,
    pub classification: String,
    pub gender_ratio: f32,
    pub evolution_chain_id: u16,
    pub relation_group_id: Option<u16>,
    pub generation_introduced: u8,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub is_baby: bool,
    pub capture_rate: u8,
    pub base_happiness: u8,
    pub growth_rate_id: u8,
    pub egg_group_1_id: u8,
    pub egg_group_2_id: Option<u8>,
    pub hatch_cycles: u8,
}
