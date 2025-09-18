use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationArea {
    pub location_area_id: i32,
    pub location_id: i32,
    pub area_name: String,
    pub area_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLocationArea {
    pub location_id: i32,
    pub area_name: String,
    pub area_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateLocationArea {
    pub area_name: Option<String>,
    pub area_identifier: Option<String>,
}
