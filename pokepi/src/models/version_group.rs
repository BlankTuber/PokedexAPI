use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VersionGroup {
    pub version_group_id: u16,
    pub version_group_name: String,
    pub version_group_identifier: String,
    pub generation: u8,
}
