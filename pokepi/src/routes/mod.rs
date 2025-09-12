use rocket::Route;

pub mod misc;

pub fn misc_routes() -> Vec<Route> {
    misc::routes()
}
