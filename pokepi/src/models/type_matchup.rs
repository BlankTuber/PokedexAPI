use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeMatchup {
    pub attacking_type_id: u8,
    pub defending_type_id: u8,
    pub damage_factor: f32,
}

pub type CreateTypeMatchup = TypeMatchup;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTypeMatchup {
    pub damage_factor: Option<f32>,
}
