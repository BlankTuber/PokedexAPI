use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AbilityDescription {
    pub ability_id: u16,
    pub version_group_id: u16,
    pub flavor_text: String,
    pub short_effect: String,
    pub effect: String,
}
