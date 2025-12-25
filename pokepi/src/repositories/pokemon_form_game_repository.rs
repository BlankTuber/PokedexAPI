use crate::error::ApiResult;
use crate::models::pokemon_form_game::{CreatePokemonFormGame, PokemonFormGame, UpdatePokemonFormGame};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct PokemonFormGameRepository;

impl PokemonFormGameRepository {
    pub async fn create(pool: &PgPool, data: CreatePokemonFormGame) -> ApiResult<PokemonFormGame> {
        let result = sqlx::query_as!(
            PokemonFormGame,
            r#"
            INSERT INTO pokemon_form_games (national_id, form_id, game_id, is_available, is_shiny_locked,
                height, weight, base_experience, hp, attack, defense, special_attack, special_defense, speed)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING pokemon_form_game_id, national_id, form_id, game_id, is_available, is_shiny_locked,
                height, weight, base_experience, hp, attack, defense, special_attack, special_defense, speed
            "#,
            data.national_id,
            data.form_id,
            data.game_id,
            data.is_available,
            data.is_shiny_locked,
            data.height,
            data.weight,
            data.base_experience,
            data.hp,
            data.attack,
            data.defense,
            data.special_attack,
            data.special_defense,
            data.speed
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<PokemonFormGame> {
        let result = sqlx::query_as!(
            PokemonFormGame,
            r#"
            SELECT pokemon_form_game_id, national_id, form_id, game_id, is_available, is_shiny_locked,
                height, weight, base_experience, hp, attack, defense, special_attack, special_defense, speed
            FROM pokemon_form_games
            WHERE pokemon_form_game_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<PokemonFormGame>> {
        let results = sqlx::query_as!(
            PokemonFormGame,
            r#"
            SELECT pokemon_form_game_id, national_id, form_id, game_id, is_available, is_shiny_locked,
                height, weight, base_experience, hp, attack, defense, special_attack, special_defense, speed
            FROM pokemon_form_games
            ORDER BY national_id, form_id, game_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_pokemon(pool: &PgPool, national_id: i32) -> ApiResult<Vec<PokemonFormGame>> {
        let results = sqlx::query_as!(
            PokemonFormGame,
            r#"
            SELECT pokemon_form_game_id, national_id, form_id, game_id, is_available, is_shiny_locked,
                height, weight, base_experience, hp, attack, defense, special_attack, special_defense, speed
            FROM pokemon_form_games
            WHERE national_id = $1
            ORDER BY form_id, game_id
            "#,
            national_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn find_by_form(pool: &PgPool, form_id: i32) -> ApiResult<Vec<PokemonFormGame>> {
        let results = sqlx::query_as!(
            PokemonFormGame,
            r#"
            SELECT pokemon_form_game_id, national_id, form_id, game_id, is_available, is_shiny_locked,
                height, weight, base_experience, hp, attack, defense, special_attack, special_defense, speed
            FROM pokemon_form_games
            WHERE form_id = $1
            ORDER BY game_id
            "#,
            form_id
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdatePokemonFormGame) -> ApiResult<()> {
        // Handle nested Option<Option<T>> for nullable stats
        let hp = match &data.hp {
            Some(Some(v)) => Some(*v),
            Some(None) => None,
            None => None,
        };
        let should_update_hp = data.hp.is_some();

        let attack = match &data.attack {
            Some(Some(v)) => Some(*v),
            Some(None) => None,
            None => None,
        };
        let should_update_attack = data.attack.is_some();

        let defense = match &data.defense {
            Some(Some(v)) => Some(*v),
            Some(None) => None,
            None => None,
        };
        let should_update_defense = data.defense.is_some();

        let special_attack = match &data.special_attack {
            Some(Some(v)) => Some(*v),
            Some(None) => None,
            None => None,
        };
        let should_update_special_attack = data.special_attack.is_some();

        let special_defense = match &data.special_defense {
            Some(Some(v)) => Some(*v),
            Some(None) => None,
            None => None,
        };
        let should_update_special_defense = data.special_defense.is_some();

        let speed = match &data.speed {
            Some(Some(v)) => Some(*v),
            Some(None) => None,
            None => None,
        };
        let should_update_speed = data.speed.is_some();

        let result = sqlx::query!(
            r#"
            UPDATE pokemon_form_games
            SET is_available = COALESCE($1, is_available),
                is_shiny_locked = COALESCE($2, is_shiny_locked),
                height = COALESCE($3, height),
                weight = COALESCE($4, weight),
                base_experience = COALESCE($5, base_experience),
                hp = CASE WHEN $6 THEN $7 ELSE hp END,
                attack = CASE WHEN $8 THEN $9 ELSE attack END,
                defense = CASE WHEN $10 THEN $11 ELSE defense END,
                special_attack = CASE WHEN $12 THEN $13 ELSE special_attack END,
                special_defense = CASE WHEN $14 THEN $15 ELSE special_defense END,
                speed = CASE WHEN $16 THEN $17 ELSE speed END
            WHERE pokemon_form_game_id = $18
            "#,
            data.is_available,
            data.is_shiny_locked,
            data.height,
            data.weight,
            data.base_experience,
            should_update_hp,
            hp,
            should_update_attack,
            attack,
            should_update_defense,
            defense,
            should_update_special_attack,
            special_attack,
            should_update_special_defense,
            special_defense,
            should_update_speed,
            speed,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form game")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM pokemon_form_games WHERE pokemon_form_game_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Pokemon form game")?;

        Ok(())
    }
}
