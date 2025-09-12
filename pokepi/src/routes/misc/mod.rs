use rocket::Route;
pub mod egg_group_handlers;

pub fn routes() -> Vec<Route> {
    routes![egg_group_handlers::create_egg_group]
}
