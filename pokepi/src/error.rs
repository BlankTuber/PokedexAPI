use rocket::serde::json::json;
use rocket::{Request, Response, http::Status, response::Responder};
use std::io::Cursor;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Conflict(String),
    BadRequest(String),
    InternalError,
}

impl ApiError {
    fn status(&self) -> Status {
        match self {
            ApiError::NotFound(_) => Status::NotFound,
            ApiError::Conflict(_) => Status::Conflict,
            ApiError::BadRequest(_) => Status::BadRequest,
            ApiError::InternalError => Status::InternalServerError,
        }
    }

    fn message(&self) -> String {
        match self {
            ApiError::NotFound(msg) => msg.clone(),
            ApiError::Conflict(msg) => msg.clone(),
            ApiError::BadRequest(msg) => msg.clone(),
            ApiError::InternalError => "Internal server error".to_string(),
        }
    }
}
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = self.status();
        let body = json!({
          "error": self.message(),
          "status": status.code
        })
        .to_string();

        Response::build()
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("Database error: {:?}", err);
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    ApiError::Conflict("Resource already exists".to_string())
                } else if db_err.is_foreign_key_violation() {
                    ApiError::Conflict(
                        "Cannot delete: resource is referenced elsewhere".to_string(),
                    )
                } else {
                    ApiError::InternalError
                }
            }
            _ => ApiError::InternalError,
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
