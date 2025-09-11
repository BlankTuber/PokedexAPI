use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub location_id: u32,
    pub location_name: String,
    pub location_identifier: String,
    pub region_id: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLocation {
    pub location_name: String,
    pub location_identifier: String,
    pub region_id: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateLocation {
    pub location_name: Option<String>,
    pub location_identifier: Option<String>,
}
