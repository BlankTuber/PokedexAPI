# Pokedex API Endpoints

## Breeding Routes (`/breeding`)

### Egg Groups

- [x] `POST /breeding/egg-groups` - Create egg group
- [x] `GET /breeding/egg-groups` - List all egg groups
- [x] `GET /breeding/egg-groups/{id}` - Get egg group by ID
- [x] `PATCH /breeding/egg-groups/{id}` - Update egg group
- [x] `DELETE /breeding/egg-groups/{id}` - Delete egg group

### Growth Rates

- [X] `POST /breeding/growth-rates` - Create growth rate
- [X] `GET /breeding/growth-rates` - List all growth rates
- [X] `GET /breeding/growth-rates/{id}` - Get growth rate by ID
- [X] `PATCH /breeding/growth-rates/{id}` - Update growth rate
- [X] `DELETE /breeding/growth-rates/{id}` - Delete growth rate

## Pokemon Routes (`/pokemon`)

### Core Pokemon

- [ ] `POST /pokemon` - Create Pokemon (requires national_id)
- [ ] `GET /pokemon` - List all Pokemon (with optional filters: generation, legendary, mythical, baby, type)
- [ ] `GET /pokemon/{national_id}` - Get Pokemon by national ID
- [ ] `PATCH /pokemon/{national_id}` - Update Pokemon
- [ ] `DELETE /pokemon/{national_id}` - Delete Pokemon

### Pokemon Forms

- [ ] `POST /pokemon/{national_id}/forms` - Create form for Pokemon
- [ ] `GET /pokemon/{national_id}/forms` - List all forms for Pokemon
- [ ] `GET /pokemon/forms/{form_id}` - Get specific form
- [ ] `PATCH /pokemon/forms/{form_id}` - Update form
- [ ] `DELETE /pokemon/forms/{form_id}` - Delete form

### Pokemon Form Games (Stats per game)

- [ ] `POST /pokemon/form-games` - Create form-game entry
- [ ] `GET /pokemon/{national_id}/games` - Get all game data for Pokemon
- [ ] `GET /pokemon/forms/{form_id}/games` - Get all game data for form
- [ ] `GET /pokemon/form-games/{pokemon_form_game_id}` - Get specific form-game entry
- [ ] `PATCH /pokemon/form-games/{pokemon_form_game_id}` - Update form-game entry
- [ ] `DELETE /pokemon/form-games/{pokemon_form_game_id}` - Delete form-game entry

### Pokemon Form Types

- [ ] `POST /pokemon/form-games/{pokemon_form_game_id}/types` - Add type to form-game
- [ ] `GET /pokemon/form-games/{pokemon_form_game_id}/types` - List types for form-game
- [ ] `PATCH /pokemon/form-games/{pokemon_form_game_id}/types/{slot}` - Update type slot
- [ ] `DELETE /pokemon/form-games/{pokemon_form_game_id}/types/{slot}` - Remove type slot

### Pokemon Form Abilities

- [ ] `POST /pokemon/form-games/{pokemon_form_game_id}/abilities` - Add ability to form-game
- [ ] `GET /pokemon/form-games/{pokemon_form_game_id}/abilities` - List abilities for form-game
- [ ] `PATCH /pokemon/form-games/{pokemon_form_game_id}/abilities/{slot}` - Update ability slot
- [ ] `DELETE /pokemon/form-games/{pokemon_form_game_id}/abilities/{slot}` - Remove ability slot

### Evolution Chains

- [ ] `POST /pokemon/evolution-chains` - Create evolution chain
- [ ] `GET /pokemon/evolution-chains` - List all evolution chains
- [ ] `GET /pokemon/evolution-chains/{id}` - Get evolution chain
- [ ] `PATCH /pokemon/evolution-chains/{id}` - Update evolution chain
- [ ] `DELETE /pokemon/evolution-chains/{id}` - Delete evolution chain

### Relation Groups

- [ ] `POST /pokemon/relation-groups` - Create relation group
- [ ] `GET /pokemon/relation-groups` - List all relation groups
- [ ] `GET /pokemon/relation-groups/{id}` - Get relation group
- [ ] `PATCH /pokemon/relation-groups/{id}` - Update relation group
- [ ] `DELETE /pokemon/relation-groups/{id}` - Delete relation group

## Combat Routes (`/combat`)

### Types

- [ ] `POST /combat/types` - Create Pokemon type
- [ ] `GET /combat/types` - List all types
- [ ] `GET /combat/types/{id}` - Get type by ID
- [ ] `PATCH /combat/types/{id}` - Update type
- [ ] `DELETE /combat/types/{id}` - Delete type

### Type Matchups

- [ ] `POST /combat/type-matchups` - Create type matchup
- [ ] `GET /combat/type-matchups` - List all matchups
- [ ] `GET /combat/type-matchups/{attacking_id}/{defending_id}` - Get specific matchup
- [ ] `PATCH /combat/type-matchups/{attacking_id}/{defending_id}` - Update matchup
- [ ] `DELETE /combat/type-matchups/{attacking_id}/{defending_id}` - Delete matchup

### Abilities

- [ ] `POST /combat/abilities` - Create ability
- [ ] `GET /combat/abilities` - List all abilities
- [ ] `GET /combat/abilities/{id}` - Get ability by ID
- [ ] `PATCH /combat/abilities/{id}` - Update ability
- [ ] `DELETE /combat/abilities/{id}` - Delete ability

### Ability Descriptions

- [ ] `POST /combat/abilities/{ability_id}/descriptions` - Create description for version group
- [ ] `GET /combat/abilities/{ability_id}/descriptions` - List all descriptions for ability
- [ ] `GET /combat/abilities/{ability_id}/descriptions/{version_group_id}` - Get specific description
- [ ] `PATCH /combat/abilities/{ability_id}/descriptions/{version_group_id}` - Update description
- [ ] `DELETE /combat/abilities/{ability_id}/descriptions/{version_group_id}` - Delete description

## Game Data Routes (`/games`)

### Games

- [ ] `POST /games` - Create game
- [ ] `GET /games` - List all games (with optional filters: generation, platform, main_series)
- [ ] `GET /games/{id}` - Get game by ID
- [ ] `PATCH /games/{id}` - Update game
- [ ] `DELETE /games/{id}` - Delete game

### Version Groups

- [ ] `POST /games/version-groups` - Create version group
- [ ] `GET /games/version-groups` - List all version groups
- [ ] `GET /games/version-groups/{id}` - Get version group
- [ ] `PATCH /games/version-groups/{id}` - Update version group
- [ ] `DELETE /games/version-groups/{id}` - Delete version group

### Platforms

- [ ] `POST /games/platforms` - Create platform
- [ ] `GET /games/platforms` - List all platforms
- [ ] `GET /games/platforms/{id}` - Get platform
- [ ] `PATCH /games/platforms/{id}` - Update platform
- [ ] `DELETE /games/platforms/{id}` - Delete platform

## World Routes (`/world`)

### Regions

- [ ] `POST /world/regions` - Create region
- [ ] `GET /world/regions` - List all regions
- [ ] `GET /world/regions/{id}` - Get region
- [ ] `PATCH /world/regions/{id}` - Update region
- [ ] `DELETE /world/regions/{id}` - Delete region

### Locations

- [ ] `POST /world/regions/{region_id}/locations` - Create location in region
- [ ] `GET /world/regions/{region_id}/locations` - List locations in region
- [ ] `GET /world/locations/{id}` - Get location
- [ ] `PATCH /world/locations/{id}` - Update location
- [ ] `DELETE /world/locations/{id}` - Delete location

### Location Areas

- [ ] `POST /world/locations/{location_id}/areas` - Create area in location
- [ ] `GET /world/locations/{location_id}/areas` - List areas in location
- [ ] `GET /world/location-areas/{id}` - Get location area
- [ ] `PATCH /world/location-areas/{id}` - Update location area
- [ ] `DELETE /world/location-areas/{id}` - Delete location area

## Encounter Routes (`/encounters`)

### Encounter Methods

- [ ] `POST /encounters/methods` - Create encounter method
- [ ] `GET /encounters/methods` - List all encounter methods
- [ ] `GET /encounters/methods/{id}` - Get encounter method
- [ ] `PATCH /encounters/methods/{id}` - Update encounter method
- [ ] `DELETE /encounters/methods/{id}` - Delete encounter method

### Encounters

- [ ] `POST /encounters` - Create encounter
- [ ] `GET /encounters` - List all encounters (with filters: pokemon, game, location, method)
- [ ] `GET /encounters/{id}` - Get encounter
- [ ] `PATCH /encounters/{id}` - Update encounter
- [ ] `DELETE /encounters/{id}` - Delete encounter

### Encounter Condition Values

- [ ] `POST /encounters/condition-values` - Create condition value
- [ ] `GET /encounters/condition-values` - List all condition values
- [ ] `GET /encounters/condition-values/{id}` - Get condition value
- [ ] `PATCH /encounters/condition-values/{id}` - Update condition value
- [ ] `DELETE /encounters/condition-values/{id}` - Delete condition value

### Encounter Conditions (Junction table)

- [ ] `POST /encounters/{encounter_id}/conditions` - Add condition to encounter
- [ ] `GET /encounters/{encounter_id}/conditions` - List conditions for encounter
- [ ] `DELETE /encounters/{encounter_id}/conditions/{condition_value_id}` - Remove condition from encounter

## Pokedex Routes (`/pokedex`)

### Regional Pokedexes

- [ ] `POST /pokedex/regional` - Create regional pokedex
- [ ] `GET /pokedex/regional` - List all regional pokedexes
- [ ] `GET /pokedex/regional/{id}` - Get regional pokedex
- [ ] `PATCH /pokedex/regional/{id}` - Update regional pokedex
- [ ] `DELETE /pokedex/regional/{id}` - Delete regional pokedex

### Pokedex Numbers

- [ ] `POST /pokedex/regional/{pokedex_id}/entries` - Add Pokemon to regional dex
- [ ] `GET /pokedex/regional/{pokedex_id}/entries` - List all entries in regional dex
- [ ] `GET /pokedex/regional/{pokedex_id}/entries/{national_id}` - Get specific entry
- [ ] `PATCH /pokedex/regional/{pokedex_id}/entries/{national_id}` - Update entry number
- [ ] `DELETE /pokedex/regional/{pokedex_id}/entries/{national_id}` - Remove from regional dex

### Pokedex Entries (Flavor text)

- [ ] `POST /pokedex/entries` - Create pokedex entry
- [ ] `GET /pokedex/entries` - List all entries (with filters: pokemon, game)
- [ ] `GET /pokemon/{national_id}/pokedex-entries` - Get all entries for Pokemon
- [ ] `GET /pokemon/{national_id}/pokedex-entries/{game_id}` - Get entry for Pokemon in specific game
- [ ] `PATCH /pokemon/{national_id}/pokedex-entries/{game_id}` - Update pokedex entry
- [ ] `DELETE /pokemon/{national_id}/pokedex-entries/{game_id}` - Delete pokedex entry

---

## Implementation Notes

- Use `- [x]` to mark completed endpoints
- Use `- [ ]` to mark incomplete endpoints
- All routes should return appropriate HTTP status codes:
  - 200 OK for successful GET/PATCH
  - 201 Created for successful POST
  - 204 No Content for successful DELETE
  - 400 Bad Request for validation errors
  - 404 Not Found for missing resources
  - 409 Conflict for duplicate entries or foreign key violations
  - 500 Internal Server Error for unexpected errors
- All endpoints should include proper error handling and validation
- Consider adding pagination for large list endpoints
- Consider adding search/filter parameters for list endpoints
