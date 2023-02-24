use crate::hashing::hash;

use crate::models::{DBUser, Db, Login, Message, ResponseMessage, Room, User};

use actix::fut::future::result;
/* use actix::Message; */
use actix_web::web::Data;

use sqlx::postgres::{PgPool, PgQueryResult};

/// `connect` takes a string, `url`, and returns a `Result<MySqlPool, sqlx::Error>`
///
/// Arguments:
///
/// * `url`: The URL to connect to the database.
///
/// Returns:
///
/// A connection pool to the database.
/* pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(url).await;

    pool
} */

/* pub async fn insert_new_user(user: User, db: &Data<Db>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO public."user" ( username, email, password, joined_at )
        VALUES ( $1, $2, $3, NOW())
        "#,
        user.username,
        user.email,
        hash(&user.password),
    )
    .execute(&db.pool)
    .await
}
 */
/* pub async fn get_user_by_username(username: &str, db: &Data<Db>) -> Result<DBUser, sqlx::Error> {
    let user = sqlx::query_as!(
        DBUser,
        r#"
        SELECT id, username FROM public."user"
        WHERE username = $1
        "#,
        username,
    )
    .fetch_one(&db.pool)
    .await?;

    Ok(user)
} */

/// It fetches all rooms for a given user
///
/// Arguments:
///
/// * `user_id`: i32 - The user id of the user we want to fetch rooms for
/// * `db`: &Data<Db>
///
/// Returns:
///
/// A vector of rooms
/* pub async fn fetch_available_rooms(user_id: i16, db: &Data<Db>) -> Result<Vec<Room>, sqlx::Error> {
    let rooms = sqlx::query_as!(
        Room,
        r#"
        SELECT room.id, name FROM room
        LEFT JOIN user_room ON user_room.id = room_id
        "#
    )
    .fetch_all(&db.pool)
    .await?;

    /*  let rooms = sqlx::query_as!(
        Room,
        r#"
        SELECT name, user_id FROM room WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?; */

    Ok(rooms)
} */

/* pub async fn send_message(
    message: String,
    room_id: i16,
    user_id: i16,
    db: &Data<Db>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO message ( message, user_id, room_id, sent_at )
        VALUES ( $1, $2, $3, NOW() )
        "#,
        message,
        user_id,
        room_id
    )
    .execute(&db.pool)
    .await?;

    Ok(())
}
 */
/* pub async fn add_user_to_room(
    room_id: i16,
    user_id: i16,
    db: &Data<Db>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_room ( user_id, room_id )
        VALUES ( $1, $2 )
        "#,
        user_id,
        room_id
    )
    .execute(&db.pool)
    .await?;

    Ok(())
} */

/* pub async fn create_new_room(name: String, user_id: i16, db: &Data<Db>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO room ( name, admin )
        VALUES ( $1, $2 )
        "#,
        name,
        &[user_id]
    )
    .execute(&db.pool)
    .await?;

    Ok(())
} */

/* pub async fn fetch_messages_by_room(
    room_id: i16,
    user_id: i16,
    db: &Data<Db>,
) -> Result<Vec<ResponseMessage>, sqlx::Error> {
    let messages = sqlx::query_as!(
        Message,
        r#"
        SELECT id, message, sent_at FROM message
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let messages = messages
        .iter()
        .map(|message| ResponseMessage {
            sent_at: message.sent_at.timestamp(),
            id: message.id,
            message: message.message.clone(),
        })
        .collect();

    Ok(messages)
} */

/* pub async fn delete_task(
    id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM tasks
            WHERE id = ? AND user_id = ?
            "#,
        id,
        user_id,
    )
    .execute(&db.pool)
    .await
} */

/* pub async fn is_an_invalid_task_id(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<bool, TaskError> {
    let task = sqlx::query!(
        r#"
        SELECT *
        FROM tasks
        WHERE id = ? AND user_id = ?
            "#,
        task_id,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(task.len() == 0)
} */

/* pub async fn start_task_and_save_time(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<ResponseTask, TaskError> {
    if is_an_invalid_task_id(task_id, user_id, db).await? {
        return Err(TaskError::InvalidId);
    }

    let task_history = sqlx::query!(
        r#"
            SELECT start_time, finish_time, task_id
            FROM task_history
            WHERE user_id = ? AND task_id = ?
            "#,
        user_id,
        task_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let is_startable =
        task_history.len() == 0 || task_history[task_history.len() - 1].finish_time.is_some();

    if is_startable {
        sqlx::query!(
            r#"
            INSERT INTO task_history ( user_id, task_id, start_time )
        VALUES ( ?, ?, NOW() )
            "#,
            user_id,
            task_id,
        )
        .execute(&db.pool)
        .await?;

        let task = fetch_task(task_id, user_id, db).await?;
        Ok(task)
    } else {
        Err(TaskError::NotFinished)
    }
} */

/* pub async fn finish_task_and_save_time(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<ResponseTask, TaskError> {
    if is_an_invalid_task_id(task_id, user_id, db).await? {
        return Err(TaskError::InvalidId);
    }

    let affected_rows = sqlx::query!(
        r#"
        UPDATE task_history
        SET finish_time = NOW()
        WHERE task_id = ? AND user_id = ? AND start_time IS NOT NULL AND finish_time IS NULL
            "#,
        task_id,
        user_id,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    if affected_rows == 1 {
        let task = fetch_task(task_id, user_id, db).await?;
        Ok(task)
    } else {
        Err(TaskError::NotStarted)
    }
} */

/* pub async fn add_task(
    user_id: i32,
    task: NewTask,
    db: &Data<Db>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO tasks ( user_id, name, description )
            VALUES ( ?, ?, ? )
            "#,
        user_id,
        task.name,
        task.description
    )
    .execute(&db.pool)
    .await
} */

/* pub async fn get_tasks_by_user(
    user_id: i32,
    db: &Data<Db>,
) -> Result<Vec<ResponseTask>, sqlx::Error> {
    let tasks = sqlx::query_as!(
        Task,
        r#"
        SELECT id, name, description FROM tasks WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let tasks_history = sqlx::query_as!(
        History,
        r#"
        SELECT task_id, start_time, finish_time FROM task_history WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let tasks: Vec<ResponseTask> = tasks
        .iter()
        .map(|task| {
            // It's creating a new ResponseTask struct and returning it.
            ResponseTask {
                id: task.id,
                name: task.name.clone(),
                description: task.description.clone(),
                history: tasks_history
                    .iter()
                    .filter(|history| history.task_id == task.id)
                    .map(|history| TaskHistory {
                        start_time: history.start_time.unix_timestamp(),
                        finish_time: match history.finish_time {
                            Some(finish_time) => Some(finish_time.unix_timestamp()),
                            None => None,
                        },
                    })
                    .collect(),
            }
        })
        .collect();

    Ok(tasks)
} */

/// It takes a `Login` struct and a `Data<Db>` struct, and returns a `Result<Vec<DBUser>, sqlx::Error>`
///
/// Arguments:
///
/// * `user`: &Login - This is the struct that we created earlier.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A vector of DBUser structs
/* pub async fn verify_user(user: &Login, db: &Data<Db>) -> Result<Vec<DBUser>, sqlx::Error> {
    let users = sqlx::query_as!(
        DBUser,
        r#"
		SELECT id, username FROM public."user"
		WHERE username = $1 AND password = $2"#,
        user.username,
        hash(&user.password)
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(users)
} */
