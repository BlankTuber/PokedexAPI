use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EncounterConditionValue {
    pub encounter_condition_value_id: u16,
    pub value_name: String,
    pub value_identifier: String,
}
