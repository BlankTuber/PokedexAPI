use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RelationGroup {
    pub relation_group_id: i16,
    pub relation_name: String,
    pub relation_identifier: String,
    pub relation_description: String,
}
