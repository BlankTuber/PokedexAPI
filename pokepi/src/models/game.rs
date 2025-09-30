use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_id: i32,
    pub game_name: String,
    pub game_identifier: String,
    pub generation: i16,
    pub version_group_id: i32,
    pub release_date: NaiveDate,
    pub platform_id: i32,
    pub is_main_series: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateGame {
    pub game_name: String,
    pub game_identifier: String,
    pub generation: i16,
    pub version_group_id: i32,
    pub release_date: NaiveDate,
    pub platform_id: i32,
    pub is_main_series: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateGame {
    pub game_name: Option<String>,
    pub game_identifier: Option<String>,
    pub generation: Option<i16>,
    pub version_group_id: Option<i32>,
    pub release_date: Option<NaiveDate>,
    pub platform_id: Option<i32>,
    pub is_main_series: Option<bool>,
}
