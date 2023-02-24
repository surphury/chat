use actix_web::web::Data;

use crate::models::{DBUser, Db};

pub async fn get_user_by_username(username: &str, db: &Data<Db>) -> Result<DBUser, sqlx::Error> {
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
}
