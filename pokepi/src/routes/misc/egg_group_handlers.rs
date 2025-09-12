use rocket::{
    State,
    http::Status,
    serde::json::{Json, Value, json},
};
use sqlx::PgPool;

use crate::models::egg_group::CreateEggGroup;

#[post("/egg-groups", format = "json", data = "<egg_group_data>")]
pub async fn create_egg_group(
    pool: &State<PgPool>,
    egg_group_data: Json<CreateEggGroup>,
) -> Result<Json<Value>, Status> {
    let egg_group = egg_group_data.into_inner();

    let result = sqlx::query!(
        r#"
        INSERT INTO egg_groups (egg_group_name, egg_group_identifier)
        VALUES ($1, $2)
        RETURNING egg_group_id, egg_group_name, egg_group_identifier
        "#,
        egg_group.egg_group_name,
        egg_group.egg_group_identifier
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => Status::Conflict,
            _ => Status::InternalServerError,
        }
    })?;

    Ok(Json(json!({
        "message": "Egg group created successfully",
        "data": {
            "egg_group_id": result.egg_group_id,
            "egg_group_name": result.egg_group_name,
            "egg_group_identifier": result.egg_group_identifier
        }
    })))
}
