use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ability {
    pub ability_id: u16,
    pub ability_name: String,
    pub ability_identifier: String,
    pub generation_introduced: u8,
    pub is_main_series: bool,
}

pub type CreateAbility = Ability;

// Route: PATCH /abilities/<ability_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateAbility {
    pub ability_name: Option<String>,
    pub ability_identifier: Option<String>,
    pub generation_introduced: Option<u8>,
    pub is_main_series: Option<bool>,
}
