use rocket::Route;
pub mod evolution_chain_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(evolution_chain_handlers::evolution_chain_routes());

    routes
}
