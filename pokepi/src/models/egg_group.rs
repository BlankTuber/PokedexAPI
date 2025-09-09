use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggGroup {
    pub egg_group_id: u8,
    pub egg_group_name: String,
    pub egg_group_identifier: String,
}

pub type CreateEggGroup = EggGroup;

// Route: PATCH /egg-groups/<egg_group_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEggGroup {
    pub egg_group_name: Option<String>,
    pub egg_group_identifier: Option<String>,
}
