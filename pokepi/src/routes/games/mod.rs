use rocket::Route;
pub mod game_handlers;
pub mod platform_handlers;
pub mod version_group_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(game_handlers::game_routes());
    routes.extend(platform_handlers::platform_routes());
    routes.extend(version_group_handlers::version_group_routes());
    routes
}
