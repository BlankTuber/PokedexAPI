use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Encounter {
    pub encounter_id: i32,
    pub national_id: i32,
    pub form_id: i32,
    pub game_id: i32,
    pub location_area_id: i32,
    pub encounter_method_id: i32,
    pub chance: i16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateEncounter {
    pub national_id: i32,
    pub form_id: i32,
    pub game_id: i32,
    pub location_area_id: i32,
    pub encounter_method_id: i32,
    pub chance: i16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEncounter {
    pub national_id: Option<i32>,
    pub form_id: Option<i32>,
    pub game_id: Option<i32>,
    pub location_area_id: Option<i32>,
    pub encounter_method_id: Option<i32>,
    pub chance: Option<i16>,
}
