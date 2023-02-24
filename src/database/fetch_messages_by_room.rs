use actix_web::web::Data;

use crate::models::{Db, Message, ResponseMessage};

pub async fn fetch_messages_by_room(
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
}
