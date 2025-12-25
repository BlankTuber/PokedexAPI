use rocket::Route;
pub mod ability_description_handlers;
pub mod ability_handlers;
pub mod pokemon_type_handlers;
pub mod type_matchup_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(ability_description_handlers::ability_description_routes());
    routes.extend(ability_handlers::ability_routes());
    routes.extend(pokemon_type_handlers::pokemon_type_routes());
    routes.extend(type_matchup_handlers::type_matchup_routes());
    routes
}
