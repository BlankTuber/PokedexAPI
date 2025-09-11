CREATE TABLE "pokemon" (
  "national_id" smallint PRIMARY KEY,
  "species_name" text NOT NULL,
  "classification" text NOT NULL,
  "gender_ratio" real NOT NULL,
  "evolution_chain_id" smallint NOT NULL,
  "relation_group_id" smallint,
  "generation_introduced" smallint NOT NULL,
  "is_legendary" boolean NOT NULL,
  "is_mythical" boolean NOT NULL,
  "is_baby" boolean NOT NULL,
  "capture_rate" smallint NOT NULL,
  "base_happiness" smallint NOT NULL,
  "growth_rate_id" smallint NOT NULL,
  "egg_group_1_id" smallint NOT NULL,
  "egg_group_2_id" smallint,
  "hatch_cycles" smallint NOT NULL  
);

CREATE TABLE "pokemon_forms" (
  "form_id" smallint PRIMARY KEY,
  "national_id" smallint NOT NULL,
  "form_name" text NOT NULL,
  "form_identifier" text NOT NULL,
  "form_type" text NOT NULL,
  "is_default" boolean NOT NULL,
  "is_battle_only" boolean NOT NULL,
  "sprite_name" text NOT NULL
);

CREATE TABLE "pokemon_form_games" (
  "pokemon_form_game_id" integer PRIMARY KEY,
  "national_id" smallint NOT NULL,
  "form_id" smallint NOT NULL,
  "game_id" smallint NOT NULL,
  "is_available" boolean NOT NULL,
  "is_shiny_locked" boolean NOT NULL,
  "height" real NOT NULL,
  "weight" real NOT NULL,
  "base_experience" smallint NOT NULL,
  "hp" smallint,
  "attack" smallint,
  "defense" smallint,
  "special_attack" smallint,
  "special_defense" smallint,
  "speed" smallint  
);

CREATE TABLE "pokemon_form_types" (
  "pokemon_form_game_id" integer NOT NULL,
  "type_id" smallint NOT NULL,
  "slot" smallint NOT NULL  
);

CREATE TABLE "pokemon_form_abilities" (
  "pokemon_form_game_id" integer NOT NULL,
  "ability_id" smallint NOT NULL,
  "slot" smallint NOT NULL,
  "is_hidden" boolean NOT NULL
);

CREATE TABLE "games" (
  "game_id" smallint PRIMARY KEY,
  "game_name" text NOT NULL,
  "game_identifier" text NOT NULL,
  "generation" smallint NOT NULL,
  "version_group_id" smallint NOT NULL,
  "release_date" date NOT NULL,
  "platform_id" smallint NOT NULL,
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "version_groups" (
  "version_group_id" smallint PRIMARY KEY,
  "version_group_name" text NOT NULL,
  "version_group_identifier" text NOT NULL,
  "generation" smallint NOT NULL  
);

CREATE TABLE "pokemon_types" (
  "type_id" smallint PRIMARY KEY,
  "type_name" text NOT NULL,
  "type_identifier" text NOT NULL,
  "generation_introduced" smallint NOT NULL
);

CREATE TABLE "type_matchups" (
  "attacking_type_id" smallint NOT NULL,
  "defending_type_id" smallint NOT NULL,
  "damage_factor" real NOT NULL
);

CREATE TABLE "abilities" (
  "ability_id" smallint PRIMARY KEY,
  "ability_name" text NOT NULL,
  "ability_identifier" text NOT NULL,
  "generation_introduced" smallint NOT NULL,
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "ability_descriptions" (
  "ability_id" smallint NOT NULL,
  "version_group_id" smallint NOT NULL,
  "flavor_text" text NOT NULL,
  "short_effect" text NOT NULL,
  "effect" text NOT NULL
);

CREATE TABLE "growth_rates" (
  "growth_rate_id" smallint PRIMARY KEY,
  "growth_rate_name" text NOT NULL,
  "growth_rate_identifier" text NOT NULL,
  "formula" text NOT NULL
);

CREATE TABLE "egg_groups" (
  "egg_group_id" smallint PRIMARY KEY,
  "egg_group_name" text NOT NULL,
  "egg_group_identifier" text NOT NULL
);

CREATE TABLE "platforms" (
  "platform_id" smallint PRIMARY KEY,
  "platform_name" text NOT NULL,
  "platform_identifier" text NOT NULL
);

CREATE TABLE "relation_groups" (
  "relation_group_id" smallint PRIMARY KEY,
  "relation_name" text NOT NULL,
  "relation_identifier" text NOT NULL,
  "relation_description" text NOT NULL
);

CREATE TABLE "regions" (
  "region_id" smallint PRIMARY KEY,
  "region_name" text NOT NULL,
  "region_identifier" text NOT NULL,
  "generation_introduced" smallint NOT NULL
);

CREATE TABLE "locations" (
  "location_id" integer PRIMARY KEY,
  "location_name" text NOT NULL,
  "location_identifier" text NOT NULL,
  "region_id" smallint NOT NULL
);

CREATE TABLE "location_areas" (
  "location_area_id" integer PRIMARY KEY,
  "location_id" integer NOT NULL,
  "area_name" text NOT NULL,
  "area_identifier" text NOT NULL
);

CREATE TABLE "encounter_methods" (
  "encounter_method_id" smallint PRIMARY KEY,
  "method_name" text NOT NULL,
  "method_identifier" text NOT NULL
);

CREATE TABLE "encounters" (
  "encounter_id" integer PRIMARY KEY,
  "national_id" smallint NOT NULL,
  "form_id" smallint NOT NULL,
  "game_id" smallint NOT NULL,
  "location_area_id" integer NOT NULL,
  "encounter_method_id" smallint NOT NULL,
  "chance" smallint NOT NULL,
);

CREATE TABLE "encounter_condition_values" (
  "encounter_condition_value_id" smallint PRIMARY KEY,
  "value_name" text NOT NULL,
  "value_identifier" text NOT NULL
);

CREATE TABLE "encounter_conditions" (
  "encounter_id" integer NOT NULL,
  "encounter_condition_value_id" smallint NOT NULL  
);

CREATE TABLE "pokedex_entries" (
  "national_id" smallint NOT NULL,
  "form_id" smallint,
  "game_id" smallint NOT NULL,
  "pokedex_number" smallint NOT NULL,
  "entry_text" text NOT NULL
);

CREATE TABLE "regional_pokedexes" (
  "pokedex_id" smallint PRIMARY KEY,
  "pokedex_name" text NOT NULL,
  "pokedex_identifier" text NOT NULL,
  "region_id" smallint NOT NULL,
  "version_group_id" smallint NOT NULL,
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "pokedex_numbers" (
  "national_id" smallint NOT NULL,
  "pokedex_id" smallint NOT NULL,
  "pokedex_number" smallint NOT NULL 
);

CREATE TABLE "evolution_chains" (
  "evolution_chain_id" smallint PRIMARY KEY
  "evolution_chain_name" text NOT NULL  
);


ALTER TABLE "pokemon" ADD FOREIGN KEY ("relation_group_id") REFERENCES "relation_groups" ("relation_group_id");
ALTER TABLE "pokemon" ADD FOREIGN KEY ("growth_rate_id") REFERENCES "growth_rates" ("growth_rate_id");
ALTER TABLE "pokemon" ADD FOREIGN KEY ("egg_group_1_id") REFERENCES "egg_groups" ("egg_group_id");
ALTER TABLE "pokemon" ADD FOREIGN KEY ("egg_group_2_id") REFERENCES "egg_groups" ("egg_group_id");
ALTER TABLE "pokemon" ADD FOREIGN KEY ("evolution_chain_id") REFERENCES "evolution_chains" ("evolution_chain_id");

ALTER TABLE "pokemon_forms" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");

ALTER TABLE "pokemon_form_games" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");
ALTER TABLE "pokemon_form_games" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_forms" ("form_id");
ALTER TABLE "pokemon_form_games" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("game_id");

ALTER TABLE "pokemon_form_types" ADD FOREIGN KEY ("pokemon_form_game_id") REFERENCES "pokemon_form_games" ("pokemon_form_game_id");
ALTER TABLE "pokemon_form_types" ADD FOREIGN KEY ("type_id") REFERENCES "pokemon_types" ("type_id");

ALTER TABLE "pokemon_form_abilities" ADD FOREIGN KEY ("pokemon_form_game_id") REFERENCES "pokemon_form_games" ("pokemon_form_game_id");
ALTER TABLE "pokemon_form_abilities" ADD FOREIGN KEY ("ability_id") REFERENCES "abilities" ("ability_id");

ALTER TABLE "games" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_groups" ("version_group_id");
ALTER TABLE "games" ADD FOREIGN KEY ("platform_id") REFERENCES "platforms" ("platform_id");

ALTER TABLE "type_matchups" ADD FOREIGN KEY ("attacking_type_id") REFERENCES "pokemon_types" ("type_id");
ALTER TABLE "type_matchups" ADD FOREIGN KEY ("defending_type_id") REFERENCES "pokemon_types" ("type_id");

ALTER TABLE "ability_descriptions" ADD FOREIGN KEY ("ability_id") REFERENCES "abilities" ("ability_id");
ALTER TABLE "ability_descriptions" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_groups" ("version_group_id");

ALTER TABLE "locations" ADD FOREIGN KEY ("region_id") REFERENCES "regions" ("region_id");
ALTER TABLE "location_areas" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("location_id");

ALTER TABLE "encounters" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");
ALTER TABLE "encounters" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_forms" ("form_id");
ALTER TABLE "encounters" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("game_id");
ALTER TABLE "encounters" ADD FOREIGN KEY ("location_area_id") REFERENCES "location_areas" ("location_area_id");
ALTER TABLE "encounters" ADD FOREIGN KEY ("encounter_method_id") REFERENCES "encounter_methods" ("encounter_method_id");

ALTER TABLE "encounter_conditions" ADD FOREIGN KEY ("encounter_id") REFERENCES "encounters" ("encounter_id");
ALTER TABLE "encounter_conditions" ADD FOREIGN KEY ("encounter_condition_value_id") REFERENCES "encounter_condition_values" ("encounter_condition_value_id");

ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");
ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_forms" ("form_id");
ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("game_id");

ALTER TABLE "regional_pokedexes" ADD FOREIGN KEY ("region_id") REFERENCES "regions" ("region_id");
ALTER TABLE "regional_pokedexes" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_groups" ("version_group_id");

ALTER TABLE "pokedex_numbers" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");
ALTER TABLE "pokedex_numbers" ADD FOREIGN KEY ("pokedex_id") REFERENCES "regional_pokedexes" ("pokedex_id");