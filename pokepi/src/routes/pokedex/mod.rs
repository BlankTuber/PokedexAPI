use rocket::Route;
pub mod pokedex_entry_handlers;
pub mod pokedex_number_handlers;
pub mod regional_pokedex_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(pokedex_entry_handlers::pokedex_entry_routes());
    routes.extend(pokedex_number_handlers::pokedex_number_routes());
    routes.extend(regional_pokedex_handlers::regional_pokedex_routes());
    routes
}
