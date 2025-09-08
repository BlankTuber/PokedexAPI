use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Game {
    pub game_id: i16,
    pub game_name: String,
    pub game_identifier: String,
    pub generation: u8,
    pub version_group_id: i16,
}
