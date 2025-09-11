use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggGroup {
    pub egg_group_id: u8,
    pub egg_group_name: String,
    pub egg_group_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateEggGroup {
    pub egg_group_name: String,
    pub egg_group_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEggGroup {
    pub egg_group_name: Option<String>,
    pub egg_group_identifier: Option<String>,
}
