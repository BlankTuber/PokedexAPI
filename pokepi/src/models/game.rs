use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_id: u16,
    pub game_name: String,
    pub game_identifier: String,
    pub generation: u8,
    pub version_group_id: u16,
    pub release_date: String,
    pub platform_id: u16,
    pub is_main_series: bool,
}

pub type CreateGame = Game;

// Route: PATCH /games/<game_id>
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateGame {
    pub game_name: Option<String>,
    pub game_identifier: Option<String>,
    pub generation: Option<u8>,
    pub version_group_id: Option<u16>,
    pub release_date: Option<String>,
    pub platform_id: Option<u16>,
    pub is_main_series: Option<bool>,
}
