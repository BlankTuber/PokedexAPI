use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    pub region_id: i32,
    pub region_name: String,
    pub region_identifier: String,
    pub generation_introduced: i16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateRegion {
    pub region_name: String,
    pub region_identifier: String,
    pub generation_introduced: i16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateRegion {
    pub region_name: Option<String>,
    pub region_identifier: Option<String>,
    pub generation_introduced: Option<i16>,
}
