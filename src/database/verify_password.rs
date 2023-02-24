use actix_web::web::Data;

use crate::models::{DBUser, Db, Login};

use crate::hashing::hash;

pub async fn verify_password(user: &Login, db: &Data<Db>) -> Result<Vec<DBUser>, sqlx::Error> {
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
}
