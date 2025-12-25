use crate::error::ApiResult;
use crate::models::game::{CreateGame, Game, UpdateGame};
use crate::repositories::game_repository::GameRepository;
use crate::validators::game_validator::GameValidator;
use sqlx::PgPool;

pub struct GameService;

impl GameService {
    pub async fn create_game(pool: &PgPool, data: CreateGame) -> ApiResult<Game> {
        GameValidator::validate_create(&data)?;
        GameRepository::create(pool, data).await
    }

    pub async fn get_game(pool: &PgPool, id: i32) -> ApiResult<Game> {
        GameRepository::find_by_id(pool, id).await
    }

    pub async fn list_games(pool: &PgPool) -> ApiResult<Vec<Game>> {
        GameRepository::find_all(pool).await
    }

    pub async fn update_game(pool: &PgPool, id: i32, data: UpdateGame) -> ApiResult<()> {
        GameValidator::validate_update(&data)?;
        GameRepository::update(pool, id, data).await
    }

    pub async fn delete_game(pool: &PgPool, id: i32) -> ApiResult<()> {
        GameRepository::delete(pool, id).await
    }
}
