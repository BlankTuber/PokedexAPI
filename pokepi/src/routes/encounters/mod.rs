use rocket::Route;
pub mod encounter_condition_handlers;
pub mod encounter_condition_value_handlers;
pub mod encounter_handlers;
pub mod encounter_method_handlers;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(encounter_condition_handlers::encounter_condition_routes());
    routes.extend(encounter_condition_value_handlers::encounter_condition_value_routes());
    routes.extend(encounter_handlers::encounter_routes());
    routes.extend(encounter_method_handlers::encounter_method_routes());
    routes
}
