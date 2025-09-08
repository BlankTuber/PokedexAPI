CREATE TABLE "pokemon" (
  "pokemon_id" integer PRIMARY KEY,
  "national_id" integer NOT NULL,
  "species_name" varchar(255) NOT NULL,
  "classification" varchar(255) NOT NULL,
  "gender_ratio" float NOT NULL,
  "evolution_chain_id" integer NOT NULL,
  "relation_group_id" integer,
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

CREATE TABLE "pokemon_form" (
  "form_id" integer PRIMARY KEY,
  "pokemon_id" integer NOT NULL,
  "form_name" varchar(255) NOT NULL,
  "form_identifier" varchar(255) NOT NULL,
  "form_type" varchar(100) NOT NULL,
  "is_default" boolean NOT NULL,
  "is_battle_only" boolean NOT NULL,
  "sprite_name" varchar(255) NOT NULL
);

CREATE TABLE "pokemon_form_game" (
  "pokemon_form_game_id" integer PRIMARY KEY,
  "pokemon_id" integer NOT NULL,
  "form_id" integer NOT NULL,
  "game_id" integer NOT NULL,
  "is_available" boolean NOT NULL,
  "is_shiny_locked" boolean NOT NULL,
  "height" float NOT NULL,
  "weight" float NOT NULL,
  "base_experience" integer NOT NULL,
  "hp" smallint,
  "attack" smallint,
  "defense" smallint,
  "special_attack" smallint,
  "special_defense" smallint,
  "speed" smallint
);

CREATE TABLE "pokemon_form_type" (
  "pokemon_form_game_id" integer NOT NULL,
  "type_id" smallint NOT NULL,
  "slot" smallint NOT NULL
);

CREATE TABLE "pokemon_form_ability" (
  "pokemon_form_game_id" integer NOT NULL,
  "ability_id" integer NOT NULL,
  "slot" smallint NOT NULL,
  "is_hidden" boolean NOT NULL
);

CREATE TABLE "game" (
  "game_id" integer PRIMARY KEY,
  "game_name" varchar(255) NOT NULL,
  "game_identifier" varchar(100) NOT NULL,
  "generation" smallint NOT NULL,
  "version_group_id" integer NOT NULL,
  "release_date" date NOT NULL,
  "platform_id" smallint NOT NULL,
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "version_group" (
  "version_group_id" integer PRIMARY KEY,
  "version_group_name" varchar(255) NOT NULL,
  "version_group_identifier" varchar(100) NOT NULL,
  "generation" smallint NOT NULL
);

CREATE TABLE "type" (
  "type_id" smallint PRIMARY KEY,
  "type_name" varchar(50) NOT NULL,
  "type_identifier" varchar(50) NOT NULL,
  "generation_introduced" smallint NOT NULL
);

CREATE TABLE "type_matchup" (
  "attacking_type_id" smallint NOT NULL,
  "defending_type_id" smallint NOT NULL,
  "damage_factor" float NOT NULL
);

CREATE TABLE "ability" (
  "ability_id" integer PRIMARY KEY,
  "ability_name" varchar(255) NOT NULL,
  "ability_identifier" varchar(100) NOT NULL,
  "generation_introduced" smallint NOT NULL,
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "ability_description" (
  "ability_id" integer NOT NULL,
  "version_group_id" integer NOT NULL,
  "flavor_text" text NOT NULL,
  "short_effect" text NOT NULL,
  "effect" text NOT NULL
);

CREATE TABLE "growth_rate" (
  "growth_rate_id" smallint PRIMARY KEY,
  "growth_rate_name" varchar(100) NOT NULL,
  "growth_rate_identifier" varchar(50) NOT NULL,
  "formula" text NOT NULL
);

CREATE TABLE "egg_group" (
  "egg_group_id" smallint PRIMARY KEY,
  "egg_group_name" varchar(100) NOT NULL,
  "egg_group_identifier" varchar(50) NOT NULL
);

CREATE TABLE "platform" (
  "platform_id" smallint PRIMARY KEY,
  "platform_name" varchar(100) NOT NULL,
  "platform_identifier" varchar(50) NOT NULL
);

CREATE TABLE "relation_group" (
  "relation_group_id" integer PRIMARY KEY,
  "relation_name" varchar(255) NOT NULL,
  "relation_identifier" varchar(100) NOT NULL,
  "relation_description" text NOT NULL
);

CREATE TABLE "region" (
  "region_id" integer PRIMARY KEY,
  "region_name" varchar(255) NOT NULL,
  "region_identifier" varchar(100) NOT NULL,
  "generation_introduced" smallint NOT NULL
);

CREATE TABLE "location" (
  "location_id" integer PRIMARY KEY,
  "location_name" varchar(255) NOT NULL,
  "location_identifier" varchar(100) NOT NULL,
  "region_id" integer NOT NULL
);

CREATE TABLE "location_area" (
  "location_area_id" integer PRIMARY KEY,
  "location_id" integer NOT NULL,
  "area_name" varchar(255) NOT NULL,
  "area_identifier" varchar(100) NOT NULL
);

CREATE TABLE "encounter_method" (
  "encounter_method_id" integer PRIMARY KEY,
  "method_name" varchar(255) NOT NULL,
  "method_identifier" varchar(100) NOT NULL,
  "order" smallint NOT NULL
);

CREATE TABLE "encounter" (
  "encounter_id" integer PRIMARY KEY,
  "pokemon_id" integer NOT NULL,
  "form_id" integer NOT NULL,
  "game_id" integer NOT NULL,
  "location_area_id" integer NOT NULL,
  "encounter_method_id" integer NOT NULL,
  "chance" smallint NOT NULL,
  "encounter_conditions_id" integer NOT NULL
);

CREATE TABLE "encounter_condition_value" (
  "encounter_condition_value_id" integer PRIMARY KEY,
  "value_name" varchar(255) NOT NULL,
  "value_identifier" varchar(100) NOT NULL
);

CREATE TABLE "encounter_condition" (
  "encounter_conditions_id" integer NOT NULL,
  "encounter_condition_value_id" integer NOT NULL
);

CREATE TABLE "pokedex_entries" (
  "pokemon_id" integer NOT NULL,
  "form_id" integer,
  "game_id" integer NOT NULL,
  "pokedex_number" smallint NOT NULL,
  "entry_text" text NOT NULL
);

CREATE TABLE "regional_pokedex" (
  "pokedex_id" integer PRIMARY KEY,
  "pokedex_name" varchar(255) NOT NULL,
  "pokedex_identifier" varchar(100) NOT NULL,
  "region_id" integer NOT NULL,
  "version_group_id" integer NOT NULL,
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "pokedex_number" (
  "pokemon_id" integer NOT NULL,
  "pokedex_id" integer NOT NULL,
  "pokedex_number" smallint NOT NULL
);

CREATE TABLE "evolution_chain" (
  "evolution_chain_id" integer PRIMARY KEY
);

ALTER TABLE "pokemon" ADD FOREIGN KEY ("relation_group_id") REFERENCES "relation_group" ("relation_group_id");

ALTER TABLE "pokemon" ADD FOREIGN KEY ("growth_rate_id") REFERENCES "growth_rate" ("growth_rate_id");

ALTER TABLE "pokemon" ADD FOREIGN KEY ("egg_group_1_id") REFERENCES "egg_group" ("egg_group_id");

ALTER TABLE "pokemon" ADD FOREIGN KEY ("egg_group_2_id") REFERENCES "egg_group" ("egg_group_id");

ALTER TABLE "pokemon_form" ADD FOREIGN KEY ("pokemon_id") REFERENCES "pokemon" ("pokemon_id");

ALTER TABLE "pokemon_form_game" ADD FOREIGN KEY ("pokemon_id") REFERENCES "pokemon" ("pokemon_id");

ALTER TABLE "pokemon_form_game" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_form" ("form_id");

ALTER TABLE "pokemon_form_game" ADD FOREIGN KEY ("game_id") REFERENCES "game" ("game_id");

ALTER TABLE "pokemon_form_type" ADD FOREIGN KEY ("pokemon_form_game_id") REFERENCES "pokemon_form_game" ("pokemon_form_game_id");

ALTER TABLE "pokemon_form_type" ADD FOREIGN KEY ("type_id") REFERENCES "type" ("type_id");

ALTER TABLE "pokemon_form_ability" ADD FOREIGN KEY ("pokemon_form_game_id") REFERENCES "pokemon_form_game" ("pokemon_form_game_id");

ALTER TABLE "pokemon_form_ability" ADD FOREIGN KEY ("ability_id") REFERENCES "ability" ("ability_id");

ALTER TABLE "game" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_group" ("version_group_id");

ALTER TABLE "game" ADD FOREIGN KEY ("platform_id") REFERENCES "platform" ("platform_id");

ALTER TABLE "type_matchup" ADD FOREIGN KEY ("attacking_type_id") REFERENCES "type" ("type_id");

ALTER TABLE "type_matchup" ADD FOREIGN KEY ("defending_type_id") REFERENCES "type" ("type_id");

ALTER TABLE "ability_description" ADD FOREIGN KEY ("ability_id") REFERENCES "ability" ("ability_id");

ALTER TABLE "ability_description" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_group" ("version_group_id");

ALTER TABLE "location" ADD FOREIGN KEY ("region_id") REFERENCES "region" ("region_id");

ALTER TABLE "location_area" ADD FOREIGN KEY ("location_id") REFERENCES "location" ("location_id");

ALTER TABLE "encounter" ADD FOREIGN KEY ("pokemon_id") REFERENCES "pokemon" ("pokemon_id");

ALTER TABLE "encounter" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_form" ("form_id");

ALTER TABLE "encounter" ADD FOREIGN KEY ("game_id") REFERENCES "game" ("game_id");

ALTER TABLE "encounter" ADD FOREIGN KEY ("location_area_id") REFERENCES "location_area" ("location_area_id");

ALTER TABLE "encounter" ADD FOREIGN KEY ("encounter_method_id") REFERENCES "encounter_method" ("encounter_method_id");

ALTER TABLE "encounter_condition" ADD FOREIGN KEY ("encounter_conditions_id") REFERENCES "encounter" ("encounter_conditions_id");

ALTER TABLE "encounter_condition" ADD FOREIGN KEY ("encounter_condition_value_id") REFERENCES "encounter_condition_value" ("encounter_condition_value_id");

ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("pokemon_id") REFERENCES "pokemon" ("pokemon_id");

ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_form" ("form_id");

ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("game_id") REFERENCES "game" ("game_id");

ALTER TABLE "regional_pokedex" ADD FOREIGN KEY ("region_id") REFERENCES "region" ("region_id");

ALTER TABLE "regional_pokedex" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_group" ("version_group_id");

ALTER TABLE "pokedex_number" ADD FOREIGN KEY ("pokemon_id") REFERENCES "pokemon" ("pokemon_id");

ALTER TABLE "pokedex_number" ADD FOREIGN KEY ("pokedex_id") REFERENCES "regional_pokedex" ("pokedex_id");

ALTER TABLE "pokemon" ADD FOREIGN KEY ("evolution_chain_id") REFERENCES "evolution_chain" ("evolution_chain_id");