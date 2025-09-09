use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EncounterCondition {
    pub encounter_conditions_id: u32,
    pub encounter_condition_value: u16,
}
