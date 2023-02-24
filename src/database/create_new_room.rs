use actix_web::web::Data;

use crate::models::Db;

pub async fn create_new_room(name: String, user_id: i16, db: &Data<Db>) -> Result<(), sqlx::Error> {
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
}
