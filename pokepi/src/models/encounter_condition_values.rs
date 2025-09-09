use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterConditionValue {
    pub encounter_condition_value_id: u16,
    pub value_name: String,
    pub value_identifier: String,
}

pub type CreateEncounterConditionValue = EncounterConditionValue;

// Route: PATCH /encounter-condition-values/<encounter_condition_value_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEncounterConditionValue {
    pub value_name: Option<String>,
    pub value_identifier: Option<String>,
}
