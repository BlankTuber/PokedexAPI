use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LocationAreas {
    pub location_area_id: u32,
    pub location_id: u32,
    pub area_name: String,
    pub area_identifier: String,
}
