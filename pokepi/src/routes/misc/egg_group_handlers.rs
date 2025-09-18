use rocket::{
    State,
    http::Status,
    serde::json::{Json, Value, json},
};
use sqlx::PgPool;

use crate::models::egg_group::{CreateEggGroup, UpdateEggGroup};

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

#[patch(
    "/egg-groups/<egg_group_id>",
    format = "json",
    data = "<updated_egg_group_data>"
)]
pub async fn update_egg_group(
    pool: &State<PgPool>,
    egg_group_id: i16,
    updated_egg_group_data: Json<UpdateEggGroup>,
) -> Result<Json<Value>, Status> {
    let updated_egg_group = updated_egg_group_data.into_inner();

    let result = sqlx::query!(
        r#"
        UPDATE egg_groups
        SET egg_group_name = COALESCE($1, egg_group_name), 
            egg_group_identifier = COALESCE($2, egg_group_identifier)
        WHERE egg_group_id = $3
        "#,
        updated_egg_group.egg_group_name,
        updated_egg_group.egg_group_identifier,
        egg_group_id,
    )
    .execute(pool.inner())
    .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                Err(Status::NotFound)
            } else {
                Ok(Json(json!({
                    "message": "Egg group updated successfully",
                })))
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
