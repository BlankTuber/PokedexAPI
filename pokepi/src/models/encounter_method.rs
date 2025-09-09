use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterMethod {
    pub encounter_method_id: u16,
    pub method_name: String,
    pub method_identifier: String,
}

pub type CreateEncounterMethod = EncounterMethod;

// Route: PATCH /encounter-methods/<encounter_method_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEncounterMethod {
    pub method_name: Option<String>,
    pub method_identifier: Option<String>,
}
