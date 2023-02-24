use actix_web::web::Data;

use crate::models::{Db, Room};

pub async fn fetch_available_rooms(user_id: i16, db: &Data<Db>) -> Result<Vec<Room>, sqlx::Error> {
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
}
