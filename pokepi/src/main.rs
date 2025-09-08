use rocket::{
    http::Status,
    serde::json::{Json, Value, json},
};

#[macro_use]
extern crate rocket;

#[get("/<name>")]
fn hello(name: &str) -> Result<Json<Value>, Status> {
    Ok(Json(json!({
      "message": format!("Hello {}", &name)
    })))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
