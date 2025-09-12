use dotenv::dotenv;
use rocket::serde::json::{Json, Value, json};
use sqlx::postgres::PgPoolOptions;

mod models;

#[macro_use]
extern crate rocket;

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
      "status": 404,
      "error": "The path you're trying to access was not found!"
    }))
}

#[catch(500)]
fn internal_error() -> Json<Value> {
    Json(json!({
      "status": 500,
      "error": "Something went wrong on our end!"
    }))
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DB_URL").expect("DB_URL is missing!"))
        .await
        .expect("Failed to connect to postgres!");

    rocket::build()
        .manage(pool)
        .register("/", catchers![not_found, internal_error])
}
