-- PostgreSQL Pokemon Database Schema
-- Optimized data types with comments for Rust mapping

CREATE TABLE "pokemon" (
  "national_id" smallint PRIMARY KEY,                    -- u16 in Rust
  "species_name" text NOT NULL,
  "classification" text NOT NULL,
  "gender_ratio" real NOT NULL,                          -- f32 in Rust  
  "evolution_chain_id" smallint NOT NULL,                -- u16 in Rust
  "relation_group_id" smallint,                          -- u16 in Rust
  "generation_introduced" smallint NOT NULL,             -- u8 in Rust (max gen ~10)
  "is_legendary" boolean NOT NULL,
  "is_mythical" boolean NOT NULL,
  "is_baby" boolean NOT NULL,
  "capture_rate" smallint NOT NULL,                      -- u8 in Rust (0-255)
  "base_happiness" smallint NOT NULL,                    -- u8 in Rust (0-255)
  "growth_rate_id" smallint NOT NULL,                    -- u8 in Rust (only ~6 growth rates)
  "egg_group_1_id" smallint NOT NULL,                    -- u8 in Rust (only ~15 egg groups)
  "egg_group_2_id" smallint,                             -- u8 in Rust (only ~15 egg groups)
  "hatch_cycles" smallint NOT NULL                       -- u8 in Rust (typically 5-120)
);

CREATE TABLE "pokemon_forms" (
  "form_id" smallint PRIMARY KEY,                        -- u16 in Rust
  "national_id" smallint NOT NULL,                       -- u16 in Rust
  "form_name" text NOT NULL,
  "form_identifier" text NOT NULL,
  "form_type" text NOT NULL,
  "is_default" boolean NOT NULL,
  "is_battle_only" boolean NOT NULL,
  "sprite_name" text NOT NULL
);

CREATE TABLE "pokemon_form_games" (
  "pokemon_form_game_id" integer PRIMARY KEY,           -- u32 in Rust (large join table)
  "national_id" smallint NOT NULL,                      -- u16 in Rust
  "form_id" smallint NOT NULL,                          -- u16 in Rust
  "game_id" smallint NOT NULL,                          -- u16 in Rust
  "is_available" boolean NOT NULL,
  "is_shiny_locked" boolean NOT NULL,
  "height" real NOT NULL,                               -- f32 in Rust
  "weight" real NOT NULL,                               -- f32 in Rust
  "base_experience" smallint NOT NULL,                  -- u16 in Rust
  "hp" smallint,                                        -- u8 in Rust (base stats 1-255)
  "attack" smallint,                                    -- u8 in Rust (base stats 1-255)
  "defense" smallint,                                   -- u8 in Rust (base stats 1-255)
  "special_attack" smallint,                            -- u8 in Rust (base stats 1-255)
  "special_defense" smallint,                           -- u8 in Rust (base stats 1-255)
  "speed" smallint                                      -- u8 in Rust (base stats 1-255)
);

CREATE TABLE "pokemon_form_types" (
  "pokemon_form_game_id" integer NOT NULL,             -- u32 in Rust
  "type_id" smallint NOT NULL,                         -- u8 in Rust (only ~18 types)
  "slot" smallint NOT NULL                             -- u8 in Rust (1 or 2)
);

CREATE TABLE "pokemon_form_abilities" (
  "pokemon_form_game_id" integer NOT NULL,             -- u32 in Rust
  "ability_id" smallint NOT NULL,                      -- u16 in Rust
  "slot" smallint NOT NULL,                            -- u8 in Rust (1, 2, or 3)
  "is_hidden" boolean NOT NULL
);

CREATE TABLE "games" (
  "game_id" smallint PRIMARY KEY,                      -- u16 in Rust
  "game_name" text NOT NULL,
  "game_identifier" text NOT NULL,
  "generation" smallint NOT NULL,                      -- u8 in Rust (max gen ~10)
  "version_group_id" smallint NOT NULL,                -- u16 in Rust
  "release_date" date NOT NULL,
  "platform_id" smallint NOT NULL,                     -- u16 in Rust
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "version_groups" (
  "version_group_id" smallint PRIMARY KEY,             -- u16 in Rust
  "version_group_name" text NOT NULL,
  "version_group_identifier" text NOT NULL,
  "generation" smallint NOT NULL                       -- u8 in Rust (max gen ~10)
);

CREATE TABLE "pokemon_types" (
  "type_id" smallint PRIMARY KEY,                      -- u8 in Rust (only ~18 types)
  "type_name" text NOT NULL,
  "type_identifier" text NOT NULL,
  "generation_introduced" smallint NOT NULL            -- u8 in Rust (max gen ~10)
);

CREATE TABLE "type_matchups" (
  "attacking_type_id" smallint NOT NULL,               -- u8 in Rust (only ~18 types)
  "defending_type_id" smallint NOT NULL,               -- u8 in Rust (only ~18 types)
  "damage_factor" real NOT NULL                        -- f32 in Rust (0.0, 0.25, 0.5, 1.0, 2.0, 4.0)
);

CREATE TABLE "abilities" (
  "ability_id" smallint PRIMARY KEY,                   -- u16 in Rust
  "ability_name" text NOT NULL,
  "ability_identifier" text NOT NULL,
  "generation_introduced" smallint NOT NULL,           -- u8 in Rust (max gen ~10)
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "ability_descriptions" (
  "ability_id" smallint NOT NULL,                      -- u16 in Rust
  "version_group_id" smallint NOT NULL,                -- u16 in Rust
  "flavor_text" text NOT NULL,
  "short_effect" text NOT NULL,
  "effect" text NOT NULL
);

CREATE TABLE "growth_rates" (
  "growth_rate_id" smallint PRIMARY KEY,               -- u8 in Rust (only ~6 growth rates)
  "growth_rate_name" text NOT NULL,
  "growth_rate_identifier" text NOT NULL,
  "formula" text NOT NULL
);

CREATE TABLE "egg_groups" (
  "egg_group_id" smallint PRIMARY KEY,                 -- u8 in Rust (only ~15 egg groups)
  "egg_group_name" text NOT NULL,
  "egg_group_identifier" text NOT NULL
);

CREATE TABLE "platforms" (
  "platform_id" smallint PRIMARY KEY,                  -- u16 in Rust
  "platform_name" text NOT NULL,
  "platform_identifier" text NOT NULL
);

CREATE TABLE "relation_groups" (
  "relation_group_id" smallint PRIMARY KEY,            -- u16 in Rust
  "relation_name" text NOT NULL,
  "relation_identifier" text NOT NULL,
  "relation_description" text NOT NULL
);

CREATE TABLE "regions" (
  "region_id" smallint PRIMARY KEY,                    -- u16 in Rust
  "region_name" text NOT NULL,
  "region_identifier" text NOT NULL,
  "generation_introduced" smallint NOT NULL            -- u8 in Rust (max gen ~10)
);

CREATE TABLE "locations" (
  "location_id" integer PRIMARY KEY,                   -- u32 in Rust (potentially large)
  "location_name" text NOT NULL,
  "location_identifier" text NOT NULL,
  "region_id" smallint NOT NULL                        -- u16 in Rust
);

CREATE TABLE "location_areas" (
  "location_area_id" integer PRIMARY KEY,              -- u32 in Rust (potentially very large)
  "location_id" integer NOT NULL,                      -- u32 in Rust
  "area_name" text NOT NULL,
  "area_identifier" text NOT NULL
);

CREATE TABLE "encounter_methods" (
  "encounter_method_id" smallint PRIMARY KEY,          -- u16 in Rust
  "method_name" text NOT NULL,
  "method_identifier" text NOT NULL
);

CREATE TABLE "encounters" (
  "encounter_id" integer PRIMARY KEY,                  -- u32 in Rust (potentially very large)
  "national_id" smallint NOT NULL,                     -- u16 in Rust
  "form_id" smallint NOT NULL,                         -- u16 in Rust
  "game_id" smallint NOT NULL,                         -- u16 in Rust
  "location_area_id" integer NOT NULL,                 -- u32 in Rust
  "encounter_method_id" smallint NOT NULL,             -- u16 in Rust
  "chance" smallint NOT NULL,                          -- u8 in Rust (0-100 percentage)
  "encounter_conditions_id" integer NOT NULL           -- u32 in Rust
);

CREATE TABLE "encounter_condition_values" (
  "encounter_condition_value_id" smallint PRIMARY KEY, -- u16 in Rust
  "value_name" text NOT NULL,
  "value_identifier" text NOT NULL
);

CREATE TABLE "encounter_conditions" (
  "encounter_conditions_id" integer NOT NULL,          -- u32 in Rust
  "encounter_condition_value_id" smallint NOT NULL     -- u16 in Rust
);

CREATE TABLE "pokedex_entries" (
  "national_id" smallint NOT NULL,                     -- u16 in Rust
  "form_id" smallint,                                  -- u16 in Rust
  "game_id" smallint NOT NULL,                         -- u16 in Rust
  "pokedex_number" smallint NOT NULL,                  -- u16 in Rust
  "entry_text" text NOT NULL
);

CREATE TABLE "regional_pokedexes" (
  "pokedex_id" smallint PRIMARY KEY,                   -- u16 in Rust
  "pokedex_name" text NOT NULL,
  "pokedex_identifier" text NOT NULL,
  "region_id" smallint NOT NULL,                       -- u16 in Rust
  "version_group_id" smallint NOT NULL,                -- u16 in Rust
  "is_main_series" boolean NOT NULL
);

CREATE TABLE "pokedex_numbers" (
  "national_id" smallint NOT NULL,                     -- u16 in Rust
  "pokedex_id" smallint NOT NULL,                      -- u16 in Rust
  "pokedex_number" smallint NOT NULL                   -- u16 in Rust
);

CREATE TABLE "evolution_chains" (
  "evolution_chain_id" smallint PRIMARY KEY            -- u16 in Rust
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

ALTER TABLE "encounter_conditions" ADD FOREIGN KEY ("encounter_conditions_id") REFERENCES "encounters" ("encounter_conditions_id");
ALTER TABLE "encounter_conditions" ADD FOREIGN KEY ("encounter_condition_value_id") REFERENCES "encounter_condition_values" ("encounter_condition_value_id");

ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");
ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("form_id") REFERENCES "pokemon_forms" ("form_id");
ALTER TABLE "pokedex_entries" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("game_id");

ALTER TABLE "regional_pokedexes" ADD FOREIGN KEY ("region_id") REFERENCES "regions" ("region_id");
ALTER TABLE "regional_pokedexes" ADD FOREIGN KEY ("version_group_id") REFERENCES "version_groups" ("version_group_id");

ALTER TABLE "pokedex_numbers" ADD FOREIGN KEY ("national_id") REFERENCES "pokemon" ("national_id");
ALTER TABLE "pokedex_numbers" ADD FOREIGN KEY ("pokedex_id") REFERENCES "regional_pokedexes" ("pokedex_id");

-- Indexes for better performance
CREATE INDEX idx_pokemon_generation ON pokemon(generation_introduced);
CREATE INDEX idx_pokemon_legendary ON pokemon(is_legendary);
CREATE INDEX idx_pokemon_mythical ON pokemon(is_mythical);

CREATE INDEX idx_pokemon_form_games_pokemon ON pokemon_form_games(national_id);
CREATE INDEX idx_pokemon_form_games_game ON pokemon_form_games(game_id);

CREATE INDEX idx_encounters_pokemon ON encounters(national_id);
CREATE INDEX idx_encounters_location ON encounters(location_area_id);
CREATE INDEX idx_encounters_game ON encounters(game_id);

CREATE INDEX idx_pokedex_entries_pokemon ON pokedex_entries(national_id);
CREATE INDEX idx_pokedex_entries_game ON pokedex_entries(game_id);