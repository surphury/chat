use actix_web::web::Data;

use crate::models::Db;

pub async fn send_message(
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
