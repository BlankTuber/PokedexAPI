use crate::error::ApiResult;
use crate::models::relation_group::{CreateRelationGroup, UpdateRelationGroup};
use crate::validators::common::CommonValidator;

pub struct RelationGroupValidator;

impl RelationGroupValidator {
    pub fn validate_create(data: &CreateRelationGroup) -> ApiResult<()> {
        CommonValidator::validate_non_empty(&data.relation_name, "Relation name")?;
        CommonValidator::validate_identifier(&data.relation_identifier, "Relation identifier")?;
        CommonValidator::validate_non_empty(&data.relation_description, "Relation description")?;
        Ok(())
    }

    pub fn validate_update(data: &UpdateRelationGroup) -> ApiResult<()> {
        CommonValidator::validate_optional_non_empty(&data.relation_name, "Relation name")?;
        CommonValidator::validate_optional_identifier(&data.relation_identifier, "Relation identifier")?;
        CommonValidator::validate_optional_non_empty(&data.relation_description, "Relation description")?;
        Ok(())
    }
}
