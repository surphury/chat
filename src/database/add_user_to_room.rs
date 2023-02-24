use actix_web::web::Data;

use crate::models::Db;

pub async fn add_user_to_room(
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
}
