use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelationGroup {
    pub relation_group_id: u16,
    pub relation_name: String,
    pub relation_identifier: String,
    pub relation_description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateRelationGroup {
    pub relation_name: String,
    pub relation_identifier: String,
    pub relation_description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateRelationGroup {
    pub relation_name: Option<String>,
    pub relation_identifier: Option<String>,
    pub relation_description: Option<String>,
}
