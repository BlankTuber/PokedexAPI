use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Platform {
    pub platform_id: u16,
    pub platform_name: String,
    pub platform_identifier: String,
}
