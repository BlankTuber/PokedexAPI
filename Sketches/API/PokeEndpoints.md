# Pokedex API Endpoints

## GET /pokedex

- Returns national pokedex
- Returns all available pokedexes

## GET /pokedex/{id}

- Returns regional dex

## GET /pokemon/{id}

- Return spesific pokemon with all related info

## GET /pokemon/{id}/*thing*

- Return spesific thing about pokemon
  - Abilities
  - Gender Ratio
  - Stats
  - Etc.

## GET /pokemon?*type*={searchTerm}

- Return pokemon based on various things such as
  - Search term
  - Type
  - Generation introduced
  - Etc.

## GET /*thing*

- Return all of spesific thing
  - Abilities
  - Types
  - Pokemon Categories
  - Etc.

## GET /*thing*/{id}

- Return deatils about spesific thing
