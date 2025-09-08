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

#[get("/pokemon")]
fn pokemon() -> Result<Json<Value>, Status> {
    Ok(Json(json!({
      "pokemon": {
        "name": "Eevee",
        "type": "Normal",
        "abilities": ["Run Away", "Adaptability"],
        "gender": 0.5,
        "height": 1.0,
        "weight": 14.3
      }
    })))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, pokemon])
}
