use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Platform {
    pub platform_id: u16,
    pub platform_name: String,
    pub platform_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatePlatform {
    pub platform_name: String,
    pub platform_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePlatform {
    pub platform_name: Option<String>,
    pub platform_identifier: Option<String>,
}
