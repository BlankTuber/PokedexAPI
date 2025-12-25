use rocket::Route;
pub mod evolution_chain_handlers;
pub mod pokemon_form_ability_handlers;
pub mod pokemon_form_game_handlers;
pub mod pokemon_form_handlers;
pub mod pokemon_form_type_handlers;
pub mod pokemon_handlers;
pub mod relation_group_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(evolution_chain_handlers::evolution_chain_routes());
    routes.extend(pokemon_form_ability_handlers::pokemon_form_ability_routes());
    routes.extend(pokemon_form_game_handlers::pokemon_form_game_routes());
    routes.extend(pokemon_form_handlers::pokemon_form_routes());
    routes.extend(pokemon_form_type_handlers::pokemon_form_type_routes());
    routes.extend(pokemon_handlers::pokemon_routes());
    routes.extend(relation_group_handlers::relation_group_routes());
    routes
}
