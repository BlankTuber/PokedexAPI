use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pub national_id: i32,
    pub species_name: String,
    pub classification: String,
    pub gender_ratio: f32,
    pub evolution_chain_id: i32,
    pub relation_group_id: Option<i32>,
    pub generation_introduced: i16,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub is_baby: bool,
    pub capture_rate: i16,
    pub base_happiness: i16,
    pub growth_rate_id: i32,
    pub egg_group_1_id: i32,
    pub egg_group_2_id: Option<i32>,
    pub hatch_cycles: i16,
}

pub type CreatePokemon = Pokemon;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePokemon {
    pub species_name: Option<String>,
    pub classification: Option<String>,
    pub gender_ratio: Option<f32>,
    pub evolution_chain_id: Option<i32>,
    pub relation_group_id: Option<Option<i32>>,
    pub generation_introduced: Option<i16>,
    pub is_legendary: Option<bool>,
    pub is_mythical: Option<bool>,
    pub is_baby: Option<bool>,
    pub capture_rate: Option<i16>,
    pub base_happiness: Option<i16>,
    pub growth_rate_id: Option<i32>,
    pub egg_group_1_id: Option<i32>,
    pub egg_group_2_id: Option<Option<i32>>,
    pub hatch_cycles: Option<i16>,
}
