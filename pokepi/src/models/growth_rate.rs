use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GrowthRate {
    pub growth_rate_id: i8,
    pub growth_rate_name: String,
    pub growth_rate_identifier: String,
    pub formula: String,
}
