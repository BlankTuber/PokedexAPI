use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionGroup {
    pub version_group_id: u16,
    pub version_group_name: String,
    pub version_group_identifier: String,
    pub generation: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateVersionGroup {
    pub version_group_name: String,
    pub version_group_identifier: String,
    pub generation: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateVersionGroup {
    pub version_group_name: Option<String>,
    pub version_group_identifier: Option<String>,
    pub generation: Option<u8>,
}
