/* use actix::{clock::Instant, Actor, Addr, AsyncContext}; */
use actix_web::HttpResponse;

use serde::{Deserialize, Serialize};

use sqlx::postgres::PgPool;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewRoom {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBUser {
    pub id: i16,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub struct Db {
    pub pool: PgPool,
}

#[derive(Deserialize, Serialize)]
pub struct NewTask {
    pub name: String,
    pub description: String,
}

pub struct Message {
    pub id: i16,
    pub message: String,
    pub sent_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct ResponseMessage {
    pub id: i16,
    pub message: String,
    pub sent_at: i64,
}

#[derive(Serialize)]
pub struct Room {
    pub id: i16,
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct TaskHistory {
    pub start_time: i64,          /* Date expressed in seconds */
    pub finish_time: Option<i64>, /* Date expressed in seconds */
}

#[derive(Serialize, Debug)]
pub struct ResponseTask {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub history: Vec<TaskHistory>,
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Claims {
    pub id_user: i16,
    pub exp: i32,
}

/* #[derive(Deserialize)]
pub struct TaskId {
    pub id: i32,
} */

/* #[derive(Clone)] */
/* pub struct History {
    pub task_id: i32,
    pub start_time: OffsetDateTime,
    pub finish_time: Option<OffsetDateTime>,
} */

/* pub enum TaskError {
    InvalidId,
    NotFinished,
    NotStarted,
    DbError(sqlx::Error),
} */

/* impl From<sqlx::Error> for TaskError {
    fn from(error: sqlx::Error) -> Self {
        TaskError::DbError(error)
    }
} */

/* impl TaskError {
    pub fn message(self) -> HttpResponse {
        match self {
            TaskError::InvalidId => {
                HttpResponse::NotFound().body("There is no task with the provided id")
            }
            TaskError::NotFinished => {
                HttpResponse::Conflict().body("The tasks hasn't been completed yet")
            }
            TaskError::NotStarted => {
                HttpResponse::Conflict().body("The tasks hasn't been started yet")
            }
            TaskError::DbError(err) => {
                println!("{:#?}", err);
                HttpResponse::InternalServerError().body("Couldn't complete operation")
            }
        }
    }
} */
pub enum VerificationError {
    InvalidToken,
    EmptyToken,
    /* ServerFailedVerifyingToken */
}

/* impl From<VerificationError> for HttpResponse {
    fn from(error: VerificationError) -> Self {
        error.message()
    }
} */

impl VerificationError {
    pub fn message(self) -> HttpResponse {
        match self {
            VerificationError::EmptyToken => {
                HttpResponse::Unauthorized().body("Empty validation token")
            }
            VerificationError::InvalidToken => HttpResponse::Unauthorized().body("Invalid Token"),
        }
    }
}
