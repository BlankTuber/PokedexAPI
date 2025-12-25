use crate::error::ApiResult;
use crate::models::pokemon::{CreatePokemon, Pokemon, UpdatePokemon};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct PokemonRepository;

impl PokemonRepository {
    pub async fn create(pool: &PgPool, data: CreatePokemon) -> ApiResult<Pokemon> {
        let result = sqlx::query_as!(
            Pokemon,
            r#"
            INSERT INTO pokemon (national_id, species_name, classification, gender_ratio, evolution_chain_id,
                relation_group_id, generation_introduced, is_legendary, is_mythical, is_baby,
                capture_rate, base_happiness, growth_rate_id, egg_group_1_id, egg_group_2_id, hatch_cycles)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING national_id, species_name, classification, gender_ratio, evolution_chain_id,
                relation_group_id, generation_introduced, is_legendary, is_mythical, is_baby,
                capture_rate, base_happiness, growth_rate_id, egg_group_1_id, egg_group_2_id, hatch_cycles
            "#,
            data.national_id,
            data.species_name,
            data.classification,
            data.gender_ratio,
            data.evolution_chain_id,
            data.relation_group_id,
            data.generation_introduced,
            data.is_legendary,
            data.is_mythical,
            data.is_baby,
            data.capture_rate,
            data.base_happiness,
            data.growth_rate_id,
            data.egg_group_1_id,
            data.egg_group_2_id,
            data.hatch_cycles
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, national_id: i32) -> ApiResult<Pokemon> {
        let pokemon = sqlx::query_as!(
            Pokemon,
            r#"
            SELECT national_id, species_name, classification, gender_ratio, evolution_chain_id,
                relation_group_id, generation_introduced, is_legendary, is_mythical, is_baby,
                capture_rate, base_happiness, growth_rate_id, egg_group_1_id, egg_group_2_id, hatch_cycles
            FROM pokemon
            WHERE national_id = $1
            "#,
            national_id
        )
        .fetch_one(pool)
        .await?;

        Ok(pokemon)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Pokemon>> {
        let pokemon = sqlx::query_as!(
            Pokemon,
            r#"
            SELECT national_id, species_name, classification, gender_ratio, evolution_chain_id,
                relation_group_id, generation_introduced, is_legendary, is_mythical, is_baby,
                capture_rate, base_happiness, growth_rate_id, egg_group_1_id, egg_group_2_id, hatch_cycles
            FROM pokemon
            ORDER BY national_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(pokemon)
    }

    pub async fn update(pool: &PgPool, national_id: i32, data: UpdatePokemon) -> ApiResult<()> {
        // Handle the nested Option<Option<i32>> for nullable foreign keys
        let relation_group_id = match &data.relation_group_id {
            Some(Some(id)) => Some(*id),
            Some(None) => None, // Explicitly set to NULL
            None => None, // No change - but we'll use COALESCE
        };
        let should_update_relation_group = data.relation_group_id.is_some();

        let egg_group_2_id = match &data.egg_group_2_id {
            Some(Some(id)) => Some(*id),
            Some(None) => None,
            None => None,
        };
        let should_update_egg_group_2 = data.egg_group_2_id.is_some();

        let result = sqlx::query!(
            r#"
            UPDATE pokemon
            SET species_name = COALESCE($1, species_name),
                classification = COALESCE($2, classification),
                gender_ratio = COALESCE($3, gender_ratio),
                evolution_chain_id = COALESCE($4, evolution_chain_id),
                relation_group_id = CASE WHEN $5 THEN $6 ELSE relation_group_id END,
                generation_introduced = COALESCE($7, generation_introduced),
                is_legendary = COALESCE($8, is_legendary),
                is_mythical = COALESCE($9, is_mythical),
                is_baby = COALESCE($10, is_baby),
                capture_rate = COALESCE($11, capture_rate),
                base_happiness = COALESCE($12, base_happiness),
                growth_rate_id = COALESCE($13, growth_rate_id),
                egg_group_1_id = COALESCE($14, egg_group_1_id),
                egg_group_2_id = CASE WHEN $15 THEN $16 ELSE egg_group_2_id END,
                hatch_cycles = COALESCE($17, hatch_cycles)
            WHERE national_id = $18
            "#,
            data.species_name,
            data.classification,
            data.gender_ratio,
            data.evolution_chain_id,
            should_update_relation_group,
            relation_group_id,
            data.generation_introduced,
            data.is_legendary,
            data.is_mythical,
            data.is_baby,
            data.capture_rate,
            data.base_happiness,
            data.growth_rate_id,
            data.egg_group_1_id,
            should_update_egg_group_2,
            egg_group_2_id,
            data.hatch_cycles,
            national_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, national_id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokemon WHERE national_id = $1
            "#,
            national_id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon")?;

        Ok(())
    }
}
