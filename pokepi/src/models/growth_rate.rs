use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrowthRate {
    pub growth_rate_id: i32,
    pub growth_rate_name: String,
    pub growth_rate_identifier: String,
    pub formula: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateGrowthRate {
    pub growth_rate_name: String,
    pub growth_rate_identifier: String,
    pub formula: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateGrowthRate {
    pub growth_rate_name: Option<String>,
    pub growth_rate_identifier: Option<String>,
    pub formula: Option<String>,
}
