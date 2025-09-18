use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ability {
    pub ability_id: i32,
    pub ability_name: String,
    pub ability_identifier: String,
    pub generation_introduced: i16,
    pub is_main_series: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateAbility {
    pub ability_name: String,
    pub ability_identifier: String,
    pub generation_introduced: i16,
    pub is_main_series: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateAbility {
    pub ability_name: Option<String>,
    pub ability_identifier: Option<String>,
    pub generation_introduced: Option<i16>,
    pub is_main_series: Option<bool>,
}
