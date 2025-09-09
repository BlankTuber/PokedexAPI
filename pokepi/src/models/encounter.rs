use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Encounter {
    pub encounter_id: u32,
    pub national_id: u16,
    pub form_id: u16,
    pub game_id: u16,
    pub location_area_id: u32,
    pub encounter_method_id: u16,
    pub chance: u8,
    pub encounter_conditions_id: u32,
}

// TODO: CRUD FOR ENCOUNTERS
