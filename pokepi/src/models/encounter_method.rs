use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterMethod {
    pub encounter_method_id: u16,
    pub method_name: String,
    pub method_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateEncounterMethod {
    pub method_name: String,
    pub method_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEncounterMethod {
    pub method_name: Option<String>,
    pub method_identifier: Option<String>,
}
