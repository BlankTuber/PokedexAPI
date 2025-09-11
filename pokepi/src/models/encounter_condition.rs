use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterCondition {
    pub encounter_id: u32,
    pub encounter_condition_value_id: u16,
}

pub type CreateEncounterCondition = EncounterCondition;
