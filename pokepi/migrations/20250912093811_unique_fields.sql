-- Migration: Add unique constraints to prevent duplicates
-- Run this after your initial schema is applied

-- Pokemon table
ALTER TABLE "pokemon" ADD CONSTRAINT unique_species_name UNIQUE ("species_name");

-- Pokemon forms
ALTER TABLE "pokemon_forms" ADD CONSTRAINT unique_form_identifier UNIQUE ("form_identifier");
ALTER TABLE "pokemon_forms" ADD CONSTRAINT unique_pokemon_form_name UNIQUE ("national_id", "form_name");

-- Pokemon form games (one record per pokemon-form-game combination)
ALTER TABLE "pokemon_form_games" ADD CONSTRAINT unique_pokemon_form_game UNIQUE ("national_id", "form_id", "game_id");

-- Pokemon form types (one type per slot per pokemon-form-game)
ALTER TABLE "pokemon_form_types" ADD CONSTRAINT unique_pokemon_form_type_slot UNIQUE ("pokemon_form_game_id", "slot");

-- Pokemon form abilities (one ability per slot per pokemon-form-game)
ALTER TABLE "pokemon_form_abilities" ADD CONSTRAINT unique_pokemon_form_ability_slot UNIQUE ("pokemon_form_game_id", "slot", "is_hidden");

-- Games
ALTER TABLE "games" ADD CONSTRAINT unique_game_identifier UNIQUE ("game_identifier");
ALTER TABLE "games" ADD CONSTRAINT unique_game_name UNIQUE ("game_name");

-- Version groups
ALTER TABLE "version_groups" ADD CONSTRAINT unique_version_group_identifier UNIQUE ("version_group_identifier");
ALTER TABLE "version_groups" ADD CONSTRAINT unique_version_group_name UNIQUE ("version_group_name");

-- Pokemon types
ALTER TABLE "pokemon_types" ADD CONSTRAINT unique_type_identifier UNIQUE ("type_identifier");
ALTER TABLE "pokemon_types" ADD CONSTRAINT unique_type_name UNIQUE ("type_name");

-- Type matchups (one matchup per type combination)
ALTER TABLE "type_matchups" ADD CONSTRAINT unique_type_matchup UNIQUE ("attacking_type_id", "defending_type_id");

-- Abilities
ALTER TABLE "abilities" ADD CONSTRAINT unique_ability_identifier UNIQUE ("ability_identifier");
ALTER TABLE "abilities" ADD CONSTRAINT unique_ability_name UNIQUE ("ability_name");

-- Ability descriptions (one description per ability per version group)
ALTER TABLE "ability_descriptions" ADD CONSTRAINT unique_ability_description UNIQUE ("ability_id", "version_group_id");

-- Growth rates
ALTER TABLE "growth_rates" ADD CONSTRAINT unique_growth_rate_identifier UNIQUE ("growth_rate_identifier");
ALTER TABLE "growth_rates" ADD CONSTRAINT unique_growth_rate_name UNIQUE ("growth_rate_name");

-- Egg groups
ALTER TABLE "egg_groups" ADD CONSTRAINT unique_egg_group_identifier UNIQUE ("egg_group_identifier");
ALTER TABLE "egg_groups" ADD CONSTRAINT unique_egg_group_name UNIQUE ("egg_group_name");

-- Platforms
ALTER TABLE "platforms" ADD CONSTRAINT unique_platform_identifier UNIQUE ("platform_identifier");
ALTER TABLE "platforms" ADD CONSTRAINT unique_platform_name UNIQUE ("platform_name");

-- Relation groups
ALTER TABLE "relation_groups" ADD CONSTRAINT unique_relation_identifier UNIQUE ("relation_identifier");
ALTER TABLE "relation_groups" ADD CONSTRAINT unique_relation_name UNIQUE ("relation_name");

-- Regions
ALTER TABLE "regions" ADD CONSTRAINT unique_region_identifier UNIQUE ("region_identifier");
ALTER TABLE "regions" ADD CONSTRAINT unique_region_name UNIQUE ("region_name");

-- Locations (unique within each region)
ALTER TABLE "locations" ADD CONSTRAINT unique_location_identifier UNIQUE ("location_identifier");
ALTER TABLE "locations" ADD CONSTRAINT unique_location_per_region UNIQUE ("region_id", "location_name");

-- Location areas (unique within each location)
ALTER TABLE "location_areas" ADD CONSTRAINT unique_area_identifier UNIQUE ("area_identifier");
ALTER TABLE "location_areas" ADD CONSTRAINT unique_area_per_location UNIQUE ("location_id", "area_name");

-- Encounter methods
ALTER TABLE "encounter_methods" ADD CONSTRAINT unique_method_identifier UNIQUE ("method_identifier");
ALTER TABLE "encounter_methods" ADD CONSTRAINT unique_method_name UNIQUE ("method_name");

-- Encounters (one encounter record per pokemon-form-game-location-method combination)
ALTER TABLE "encounters" ADD CONSTRAINT unique_encounter UNIQUE ("national_id", "form_id", "game_id", "location_area_id", "encounter_method_id");

-- Encounter condition values
ALTER TABLE "encounter_condition_values" ADD CONSTRAINT unique_condition_value_identifier UNIQUE ("value_identifier");
ALTER TABLE "encounter_condition_values" ADD CONSTRAINT unique_condition_value_name UNIQUE ("value_name");

-- Encounter conditions (one condition per encounter)
ALTER TABLE "encounter_conditions" ADD CONSTRAINT unique_encounter_condition UNIQUE ("encounter_id", "encounter_condition_value_id");

-- Pokedex entries (one entry per pokemon-form-game combination)
ALTER TABLE "pokedex_entries" ADD CONSTRAINT unique_pokedex_entry UNIQUE ("national_id", "form_id", "game_id");

-- Regional pokedexes
ALTER TABLE "regional_pokedexes" ADD CONSTRAINT unique_pokedex_identifier UNIQUE ("pokedex_identifier");
ALTER TABLE "regional_pokedexes" ADD CONSTRAINT unique_pokedex_per_region_version UNIQUE ("region_id", "version_group_id", "pokedex_name");

-- Pokedex numbers (one number per pokemon per pokedex)
ALTER TABLE "pokedex_numbers" ADD CONSTRAINT unique_pokedex_number UNIQUE ("national_id", "pokedex_id");
ALTER TABLE "pokedex_numbers" ADD CONSTRAINT unique_number_per_pokedex UNIQUE ("pokedex_id", "pokedex_number");

-- Evolution chains
ALTER TABLE "evolution_chains" ADD CONSTRAINT unique_evolution_chain_name UNIQUE ("evolution_chain_name");