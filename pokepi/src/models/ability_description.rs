use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AbilityDescription {
    pub ability_id: i32,
    pub version_group_id: i32,
    pub flavor_text: String,
    pub short_effect: String,
    pub effect: String,
}

pub type CreateAbilityDescription = AbilityDescription;

// Route: PATCH /abilities/<ability_id>/descriptions/<version_group_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateAbilityDescription {
    pub flavor_text: Option<String>,
    pub short_effect: Option<String>,
    pub effect: Option<String>,
}
