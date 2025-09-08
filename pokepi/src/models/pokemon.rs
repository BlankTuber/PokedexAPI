use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pokemon {
    pub national_id: i16,
    pub species_name: String,
    pub classification: String,
    pub gender_ratio: f64,
    pub evolution_chain_id: i16,
    pub relation_group_id: Option<i16>,
    pub generation_introduced: i8,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub is_baby: bool,
    pub capture_rate: i8,
    pub base_happiness: i8,
    pub growth_rate_id: i8,
    pub egg_group_1_id: i8,
    pub egg_group_2_id: Option<i8>,
    pub hatch_cycles: i8,
}
