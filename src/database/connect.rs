use sqlx::PgPool;

pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(url).await;

    pool
}
