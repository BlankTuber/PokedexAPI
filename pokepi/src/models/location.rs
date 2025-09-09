use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Location {
    pub location_id: u32,
    pub location_name: String,
    pub location_identifier: String,
    pub region_id: u16,
}
