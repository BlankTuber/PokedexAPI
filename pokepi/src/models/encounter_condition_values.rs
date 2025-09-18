use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterConditionValue {
    pub encounter_condition_value_id: i32,
    pub value_name: String,
    pub value_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateEncounterConditionValue {
    pub value_name: String,
    pub value_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEncounterConditionValue {
    pub value_name: Option<String>,
    pub value_identifier: Option<String>,
}
