use crate::error::ApiResult;
use crate::models::relation_group::{CreateRelationGroup, RelationGroup, UpdateRelationGroup};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct RelationGroupRepository;

impl RelationGroupRepository {
    pub async fn create(pool: &PgPool, data: CreateRelationGroup) -> ApiResult<RelationGroup> {
        let result = sqlx::query_as!(
            RelationGroup,
            r#"
            INSERT INTO relation_groups (relation_name, relation_identifier, relation_description)
            VALUES ($1, $2, $3)
            RETURNING relation_group_id, relation_name, relation_identifier, relation_description
            "#,
            data.relation_name,
            data.relation_identifier,
            data.relation_description
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<RelationGroup> {
        let result = sqlx::query_as!(
            RelationGroup,
            r#"
            SELECT relation_group_id, relation_name, relation_identifier, relation_description
            FROM relation_groups
            WHERE relation_group_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<RelationGroup>> {
        let results = sqlx::query_as!(
            RelationGroup,
            r#"
            SELECT relation_group_id, relation_name, relation_identifier, relation_description
            FROM relation_groups
            ORDER BY relation_group_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateRelationGroup) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE relation_groups
            SET relation_name = COALESCE($1, relation_name),
                relation_identifier = COALESCE($2, relation_identifier),
                relation_description = COALESCE($3, relation_description)
            WHERE relation_group_id = $4
            "#,
            data.relation_name,
            data.relation_identifier,
            data.relation_description,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Relation group")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM relation_groups WHERE relation_group_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Relation group")?;

        Ok(())
    }
}
