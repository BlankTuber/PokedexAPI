use crate::error::ApiResult;
use crate::models::relation_group::{CreateRelationGroup, RelationGroup, UpdateRelationGroup};
use crate::services::relation_group_service::RelationGroupService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/relation-groups", format = "json", data = "<data>")]
pub async fn create_relation_group(
    pool: &State<PgPool>,
    data: Json<CreateRelationGroup>,
) -> ApiResult<Json<RelationGroup>> {
    let result = RelationGroupService::create(pool.inner(), data.into_inner()).await?;
    Ok(Json(result))
}

#[get("/relation-groups/<id>")]
pub async fn get_relation_group(pool: &State<PgPool>, id: i32) -> ApiResult<Json<RelationGroup>> {
    let result = RelationGroupService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[get("/relation-groups")]
pub async fn list_relation_groups(pool: &State<PgPool>) -> ApiResult<Json<Vec<RelationGroup>>> {
    let results = RelationGroupService::list(pool.inner()).await?;
    Ok(Json(results))
}

#[patch("/relation-groups/<id>", format = "json", data = "<data>")]
pub async fn update_relation_group(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateRelationGroup>,
) -> ApiResult<Json<RelationGroup>> {
    RelationGroupService::update(pool.inner(), id, data.into_inner()).await?;
    let result = RelationGroupService::get(pool.inner(), id).await?;
    Ok(Json(result))
}

#[delete("/relation-groups/<id>")]
pub async fn delete_relation_group(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    RelationGroupService::delete(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn relation_group_routes() -> Vec<Route> {
    routes![
        create_relation_group,
        get_relation_group,
        list_relation_groups,
        update_relation_group,
        delete_relation_group
    ]
}
