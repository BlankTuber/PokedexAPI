use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TypeMatchup {
    pub attacking_type_id: i16,
    pub defending_type_id: i16,
    pub damage_factor: f64,
}
