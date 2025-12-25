use rocket::Route;

pub mod pokemon;
// pub mod combat;
pub mod breeding;
pub mod games;
// pub mod world;
// pub mod encounters;
// pub mod pokedex;

pub fn pokemon_routes() -> Vec<Route> {
    pokemon::routes()
}

// pub fn combat_routes() -> Vec<Route> {
//     combat::routes()
// }

pub fn breeding_routes() -> Vec<Route> {
    breeding::routes()
}

pub fn games_routes() -> Vec<Route> {
    games::routes()
}

// pub fn world_routes() -> Vec<Route> {
//     world::routes()
// }

// pub fn encounter_routes() -> Vec<Route> {
//     encounters::routes()
// }

// pub fn pokedex_routes() -> Vec<Route> {
//     pokedex::routes()
// }
