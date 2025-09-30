use rocket::Route;
pub mod egg_group_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(egg_group_handlers::egg_group_routes());

    routes
}
