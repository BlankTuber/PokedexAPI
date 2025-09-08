use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EncounterMethod {
    pub encounter_method_id: i16,
    pub method_name: String,
    pub method_identifier: String,
}
