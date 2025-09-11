use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationArea {
    pub location_area_id: u32,
    pub location_id: u32,
    pub area_name: String,
    pub area_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLocationArea {
    pub location_id: u32,
    pub area_name: String,
    pub area_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateLocationArea {
    pub area_name: Option<String>,
    pub area_identifier: Option<String>,
}
