use actix_web::web::Data;
use sqlx::postgres::PgQueryResult;

use crate::hashing::hash;
use crate::models::{Db, User};

pub async fn insert_new_user(user: User, db: &Data<Db>) -> Result<PgQueryResult, sqlx::Error> {
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
