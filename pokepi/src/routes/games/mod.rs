use rocket::Route;
pub mod platform_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(platform_handlers::platform_routes());
    routes
}
