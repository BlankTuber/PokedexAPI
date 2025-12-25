use rocket::Route;
pub mod location_area_handlers;
pub mod location_handlers;
pub mod region_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(location_area_handlers::location_area_routes());
    routes.extend(location_handlers::location_routes());
    routes.extend(region_handlers::region_routes());
    routes
}
