-- Core Pokemon data
CREATE TABLE "pokemon" (
  "national_id" INTEGER PRIMARY KEY,
  "species_name" TEXT NOT NULL,
  "classification" TEXT NOT NULL,
  "gender_ratio" REAL NOT NULL,
  "evolution_chain_id" INTEGER NOT NULL,
  "relation_group_id" INTEGER,
  "generation_introduced" SMALLINT NOT NULL,
  "is_legendary" BOOLEAN NOT NULL,
  "is_mythical" BOOLEAN NOT NULL,
  "is_baby" BOOLEAN NOT NULL,
  "capture_rate" SMALLINT NOT NULL,
  "base_happiness" SMALLINT NOT NULL,
  "growth_rate_id" INTEGER NOT NULL,
  "egg_group_1_id" INTEGER NOT NULL,
  "egg_group_2_id" INTEGER,
  "hatch_cycles" SMALLINT NOT NULL,
  CONSTRAINT unique_species_name UNIQUE ("species_name")
);

-- Pokemon forms (different appearances/variants)
CREATE TABLE "pokemon_forms" (
  "form_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "national_id" INTEGER NOT NULL,
  "form_name" TEXT NOT NULL,
  "form_identifier" TEXT NOT NULL,
  "form_type" TEXT NOT NULL,
  "is_default" BOOLEAN NOT NULL,
  "is_battle_only" BOOLEAN NOT NULL,
  "sprite_name" TEXT NOT NULL,
  CONSTRAINT unique_form_identifier UNIQUE ("form_identifier"),
  CONSTRAINT unique_pokemon_form_name UNIQUE ("national_id", "form_name")
);

-- Pokemon form data per game (stats, availability)
CREATE TABLE "pokemon_form_games" (
  "pokemon_form_game_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "national_id" INTEGER NOT NULL,
  "form_id" INTEGER NOT NULL,
  "game_id" INTEGER NOT NULL,
  "is_available" BOOLEAN NOT NULL,
  "is_shiny_locked" BOOLEAN NOT NULL,
  "height" REAL NOT NULL,
  "weight" REAL NOT NULL,
  "base_experience" INTEGER NOT NULL,
  "hp" SMALLINT,
  "attack" SMALLINT,
  "defense" SMALLINT,
  "special_attack" SMALLINT,
  "special_defense" SMALLINT,
  "speed" SMALLINT,
  CONSTRAINT unique_pokemon_form_game UNIQUE ("national_id", "form_id", "game_id")
);

-- Types for each pokemon form per game
CREATE TABLE "pokemon_form_types" (
  "pokemon_form_game_id" INTEGER NOT NULL,
  "type_id" INTEGER NOT NULL,
  "slot" SMALLINT NOT NULL,
  CONSTRAINT unique_pokemon_form_type_slot UNIQUE ("pokemon_form_game_id", "slot")
);

-- Abilities for each pokemon form per game
CREATE TABLE "pokemon_form_abilities" (
  "pokemon_form_game_id" INTEGER NOT NULL,
  "ability_id" INTEGER NOT NULL,
  "slot" SMALLINT NOT NULL,
  "is_hidden" BOOLEAN NOT NULL,
  CONSTRAINT unique_pokemon_form_ability_slot UNIQUE ("pokemon_form_game_id", "slot", "is_hidden")
);

-- Game information
CREATE TABLE "games" (
  "game_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "game_name" TEXT NOT NULL,
  "game_identifier" TEXT NOT NULL,
  "generation" SMALLINT NOT NULL,
  "version_group_id" INTEGER NOT NULL,
  "release_date" DATE NOT NULL,
  "platform_id" INTEGER NOT NULL,
  "is_main_series" BOOLEAN NOT NULL,
  CONSTRAINT unique_game_identifier UNIQUE ("game_identifier"),
  CONSTRAINT unique_game_name UNIQUE ("game_name")
);

-- Version groups (Red/Blue, Gold/Silver, etc.)
CREATE TABLE "version_groups" (
  "version_group_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "version_group_name" TEXT NOT NULL,
  "version_group_identifier" TEXT NOT NULL,
  "generation" SMALLINT NOT NULL,
  CONSTRAINT unique_version_group_identifier UNIQUE ("version_group_identifier"),
  CONSTRAINT unique_version_group_name UNIQUE ("version_group_name")
);

-- Pokemon types (Fire, Water, etc.)
CREATE TABLE "pokemon_types" (
  "type_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "type_name" TEXT NOT NULL,
  "type_identifier" TEXT NOT NULL,
  "generation_introduced" SMALLINT NOT NULL,
  CONSTRAINT unique_type_identifier UNIQUE ("type_identifier"),
  CONSTRAINT unique_type_name UNIQUE ("type_name")
);

-- Type effectiveness chart
CREATE TABLE "type_matchups" (
  "attacking_type_id" INTEGER NOT NULL,
  "defending_type_id" INTEGER NOT NULL,
  "damage_factor" REAL NOT NULL,
  CONSTRAINT unique_type_matchup UNIQUE ("attacking_type_id", "defending_type_id")
);

-- Abilities
CREATE TABLE "abilities" (
  "ability_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "ability_name" TEXT NOT NULL,
  "ability_identifier" TEXT NOT NULL,
  "generation_introduced" SMALLINT NOT NULL,
  "is_main_series" BOOLEAN NOT NULL,
  CONSTRAINT unique_ability_identifier UNIQUE ("ability_identifier"),
  CONSTRAINT unique_ability_name UNIQUE ("ability_name")
);

-- Ability descriptions per version
CREATE TABLE "ability_descriptions" (
  "ability_id" INTEGER NOT NULL,
  "version_group_id" INTEGER NOT NULL,
  "flavor_text" TEXT NOT NULL,
  "short_effect" TEXT NOT NULL,
  "effect" TEXT NOT NULL,
  CONSTRAINT unique_ability_description UNIQUE ("ability_id", "version_group_id")
);

-- Experience growth rates
CREATE TABLE "growth_rates" (
  "growth_rate_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "growth_rate_name" TEXT NOT NULL,
  "growth_rate_identifier" TEXT NOT NULL,
  "formula" TEXT NOT NULL,
  CONSTRAINT unique_growth_rate_identifier UNIQUE ("growth_rate_identifier"),
  CONSTRAINT unique_growth_rate_name UNIQUE ("growth_rate_name")
);

-- Egg groups for breeding
CREATE TABLE "egg_groups" (
  "egg_group_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "egg_group_name" TEXT NOT NULL,
  "egg_group_identifier" TEXT NOT NULL,
  CONSTRAINT unique_egg_group_identifier UNIQUE ("egg_group_identifier"),
  CONSTRAINT unique_egg_group_name UNIQUE ("egg_group_name")
);

-- Gaming platforms
CREATE TABLE "platforms" (
  "platform_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "platform_name" TEXT NOT NULL,
  "platform_identifier" TEXT NOT NULL,
  CONSTRAINT unique_platform_identifier UNIQUE ("platform_identifier"),
  CONSTRAINT unique_platform_name UNIQUE ("platform_name")
);

-- Pokemon relationships (baby/evolution groups)
CREATE TABLE "relation_groups" (
  "relation_group_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "relation_name" TEXT NOT NULL,
  "relation_identifier" TEXT NOT NULL,
  "relation_description" TEXT NOT NULL,
  CONSTRAINT unique_relation_identifier UNIQUE ("relation_identifier"),
  CONSTRAINT unique_relation_name UNIQUE ("relation_name")
);

-- Game regions
CREATE TABLE "regions" (
  "region_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "region_name" TEXT NOT NULL,
  "region_identifier" TEXT NOT NULL,
  "generation_introduced" SMALLINT NOT NULL,
  CONSTRAINT unique_region_identifier UNIQUE ("region_identifier"),
  CONSTRAINT unique_region_name UNIQUE ("region_name")
);

-- Locations within regions
CREATE TABLE "locations" (
  "location_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "location_name" TEXT NOT NULL,
  "location_identifier" TEXT NOT NULL,
  "region_id" INTEGER NOT NULL,
  CONSTRAINT unique_location_identifier UNIQUE ("location_identifier"),
  CONSTRAINT unique_location_per_region UNIQUE ("region_id", "location_name")
);

-- Specific areas within locations
CREATE TABLE "location_areas" (
  "location_area_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "location_id" INTEGER NOT NULL,
  "area_name" TEXT NOT NULL,
  "area_identifier" TEXT NOT NULL,
  CONSTRAINT unique_area_identifier UNIQUE ("area_identifier"),
  CONSTRAINT unique_area_per_location UNIQUE ("location_id", "area_name")
);

-- Ways to encounter Pokemon
CREATE TABLE "encounter_methods" (
  "encounter_method_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "method_name" TEXT NOT NULL,
  "method_identifier" TEXT NOT NULL,
  CONSTRAINT unique_method_identifier UNIQUE ("method_identifier"),
  CONSTRAINT unique_method_name UNIQUE ("method_name")
);

-- Pokemon encounters in specific locations
CREATE TABLE "encounters" (
  "encounter_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "national_id" INTEGER NOT NULL,
  "form_id" INTEGER NOT NULL,
  "game_id" INTEGER NOT NULL,
  "location_area_id" INTEGER NOT NULL,
  "encounter_method_id" INTEGER NOT NULL,
  "chance" SMALLINT NOT NULL,
  CONSTRAINT unique_encounter UNIQUE ("national_id", "form_id", "game_id", "location_area_id", "encounter_method_id")
);

-- Conditions for encounters (time of day, weather, etc.)
CREATE TABLE "encounter_condition_values" (
  "encounter_condition_value_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "value_name" TEXT NOT NULL,
  "value_identifier" TEXT NOT NULL,
  CONSTRAINT unique_condition_value_identifier UNIQUE ("value_identifier"),
  CONSTRAINT unique_condition_value_name UNIQUE ("value_name")
);

-- Link encounters to their conditions
CREATE TABLE "encounter_conditions" (
  "encounter_id" INTEGER NOT NULL,
  "encounter_condition_value_id" INTEGER NOT NULL,
  CONSTRAINT unique_encounter_condition UNIQUE ("encounter_id", "encounter_condition_value_id")
);

-- Pokedex entries (flavor text)
CREATE TABLE "pokedex_entries" (
  "national_id" INTEGER NOT NULL,
  "form_id" INTEGER,
  "game_id" INTEGER NOT NULL,
  "pokedex_number" INTEGER NOT NULL,
  "entry_text" TEXT NOT NULL,
  CONSTRAINT unique_pokedex_entry UNIQUE ("national_id", "form_id", "game_id")
);

-- Regional pokedexes (Kanto, Johto, etc.)
CREATE TABLE "regional_pokedexes" (
  "pokedex_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "pokedex_name" TEXT NOT NULL,
  "pokedex_identifier" TEXT NOT NULL,
  "region_id" INTEGER NOT NULL,
  "version_group_id" INTEGER NOT NULL,
  "is_main_series" BOOLEAN NOT NULL,
  CONSTRAINT unique_pokedex_identifier UNIQUE ("pokedex_identifier"),
  CONSTRAINT unique_pokedex_per_region_version UNIQUE ("region_id", "version_group_id", "pokedex_name")
);

-- Pokemon numbers in regional dexes
CREATE TABLE "pokedex_numbers" (
  "national_id" INTEGER NOT NULL,
  "pokedex_id" INTEGER NOT NULL,
  "pokedex_number" INTEGER NOT NULL,
  CONSTRAINT unique_pokedex_number UNIQUE ("national_id", "pokedex_id"),
  CONSTRAINT unique_number_per_pokedex UNIQUE ("pokedex_id", "pokedex_number")
);

-- Evolution chains
CREATE TABLE "evolution_chains" (
  "evolution_chain_id" INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "evolution_chain_name" TEXT NOT NULL,
  CONSTRAINT unique_evolution_chain_name UNIQUE ("evolution_chain_name")
);

-- Foreign Key Constraints
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

-- Indexes for performance (optional but recommended)
CREATE INDEX idx_pokemon_generation ON "pokemon" ("generation_introduced");
CREATE INDEX idx_pokemon_legendary ON "pokemon" ("is_legendary");
CREATE INDEX idx_pokemon_mythical ON "pokemon" ("is_mythical");
CREATE INDEX idx_forms_national_id ON "pokemon_forms" ("national_id");
CREATE INDEX idx_form_games_national_id ON "pokemon_form_games" ("national_id");
CREATE INDEX idx_form_games_game_id ON "pokemon_form_games" ("game_id");
CREATE INDEX idx_encounters_national_id ON "encounters" ("national_id");
CREATE INDEX idx_encounters_location ON "encounters" ("location_area_id");
CREATE INDEX idx_encounters_method ON "encounters" ("encounter_method_id");