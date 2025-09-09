use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Region {
    pub region_id: u16,
    pub region_name: String,
    pub region_identifier: String,
    pub generation_introduced: u8,
}
