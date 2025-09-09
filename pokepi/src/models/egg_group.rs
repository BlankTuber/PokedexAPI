use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EggGroup {
    pub egg_group_id: u8,
    pub egg_group_name: String,
    pub egg_group_identifier: String,
}
