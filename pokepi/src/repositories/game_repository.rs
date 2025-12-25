use crate::error::ApiResult;
use crate::models::game::{CreateGame, Game, UpdateGame};
use crate::validators::common::CommonValidator;
use sqlx::PgPool;

pub struct GameRepository;

impl GameRepository {
    pub async fn create(pool: &PgPool, data: CreateGame) -> ApiResult<Game> {
        let result = sqlx::query_as!(
            Game,
            r#"
            INSERT INTO games (game_name, game_identifier, generation, version_group_id, release_date, platform_id, is_main_series)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING game_id, game_name, game_identifier, generation, version_group_id, release_date, platform_id, is_main_series
            "#,
            data.game_name,
            data.game_identifier,
            data.generation,
            data.version_group_id,
            data.release_date,
            data.platform_id,
            data.is_main_series
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> ApiResult<Game> {
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT game_id, game_name, game_identifier, generation, version_group_id, release_date, platform_id, is_main_series
            FROM games
            WHERE game_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(game)
    }

    pub async fn find_all(pool: &PgPool) -> ApiResult<Vec<Game>> {
        let games = sqlx::query_as!(
            Game,
            r#"
            SELECT game_id, game_name, game_identifier, generation, version_group_id, release_date, platform_id, is_main_series
            FROM games
            ORDER BY game_id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(games)
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdateGame) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE games
            SET game_name = COALESCE($1, game_name),
                game_identifier = COALESCE($2, game_identifier),
                generation = COALESCE($3, generation),
                version_group_id = COALESCE($4, version_group_id),
                release_date = COALESCE($5, release_date),
                platform_id = COALESCE($6, platform_id),
                is_main_series = COALESCE($7, is_main_series)
            WHERE game_id = $8
            "#,
            data.game_name,
            data.game_identifier,
            data.generation,
            data.version_group_id,
            data.release_date,
            data.platform_id,
            data.is_main_series,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Game")?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM games WHERE game_id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        CommonValidator::validate_rows_affected(result.rows_affected(), "Game")?;

        Ok(())
    }
}
