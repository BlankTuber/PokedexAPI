use crate::error::ApiResult;
use crate::models::game::{CreateGame, Game, UpdateGame};
use crate::services::game_service::GameService;
use rocket::http::Status;
use rocket::{Route, State, serde::json::Json};
use sqlx::PgPool;

#[post("/games", format = "json", data = "<data>")]
pub async fn create_game(
    pool: &State<PgPool>,
    data: Json<CreateGame>,
) -> ApiResult<Json<Game>> {
    let game = GameService::create_game(pool.inner(), data.into_inner()).await?;
    Ok(Json(game))
}

#[get("/games/<id>")]
pub async fn get_game(pool: &State<PgPool>, id: i32) -> ApiResult<Json<Game>> {
    let game = GameService::get_game(pool.inner(), id).await?;
    Ok(Json(game))
}

#[get("/games")]
pub async fn list_games(pool: &State<PgPool>) -> ApiResult<Json<Vec<Game>>> {
    let games = GameService::list_games(pool.inner()).await?;
    Ok(Json(games))
}

#[patch("/games/<id>", format = "json", data = "<data>")]
pub async fn update_game(
    pool: &State<PgPool>,
    id: i32,
    data: Json<UpdateGame>,
) -> ApiResult<Json<Game>> {
    GameService::update_game(pool.inner(), id, data.into_inner()).await?;
    let game = GameService::get_game(pool.inner(), id).await?;
    Ok(Json(game))
}

#[delete("/games/<id>")]
pub async fn delete_game(pool: &State<PgPool>, id: i32) -> ApiResult<Status> {
    GameService::delete_game(pool.inner(), id).await?;
    Ok(Status::NoContent)
}

pub fn game_routes() -> Vec<Route> {
    routes![
        create_game,
        get_game,
        list_games,
        update_game,
        delete_game
    ]
}
