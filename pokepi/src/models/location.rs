use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Location {
    pub location_id: u16,
    pub location_name: String,
    pub location_identifier: String,
    pub region_id: i16,
}
