use sqlx::PgPool;

pub async fn count_symbols(pool: &PgPool) -> sqlx::Result<i64> {
    sqlx::query_scalar("SELECT COUNT(*) FROM wisdom.symbols")
        .fetch_one(pool)
        .await
}
