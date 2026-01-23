# PokePI

REST API for managing Pokémon data. Covers pokemon, forms, types, abilities, games, regions, encounters, breeding info, and pokedex entries.

## Routes

| Base Path     | What it covers                                                |
|---------------|---------------------------------------------------------------|
| `/breeding`   | Egg groups, growth rates                                      |
| `/pokemon`    | Pokemon, forms, form-games, relation groups, evolution chains |
| `/combat`     | Types, abilities, type matchups, ability descriptions         |
| `/games`      | Games, platforms, version groups                              |
| `/world`      | Regions, locations, location areas                            |
| `/encounters` | Encounters, methods, conditions                               |
| `/pokedex`    | Pokedex entries, regional pokedexes, pokedex numbers          |

All resources support standard CRUD operations (POST, GET, PATCH, DELETE).

## Tech

- Rust
- Rocket (web framework)
- SQLx (database)
- PostgreSQL

## Notice

- After realizing the scale of the project, and my desire to explore outside of APIs, I let Claude Code (AI Agent) help with the majority of the routes, tho it worked based on my original few routes.
- Feel free to use the API however you want, but I will personally not host it.
