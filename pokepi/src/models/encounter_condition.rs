use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterCondition {
    pub encounter_id: i32,
    pub encounter_condition_value_id: i32,
}

pub type CreateEncounterCondition = EncounterCondition;
