use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Symbol {
    pub id: Uuid,
    pub title: String,
    pub formula: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewSymbol {
    pub title: String,
    pub formula: String,
}

pub async fn create_symbol(pool: &PgPool, symbol: NewSymbol) -> sqlx::Result<Symbol> {
    let NewSymbol { title, formula } = symbol;

    sqlx::query_as!(
        Symbol,
        "
INSERT INTO wisdom.symbols (id, title, formula)
VALUES ($1, $2, $3)
RETURNING id, title, formula, created_at, updated_at
        ",
        Uuid::now_v7(),
        title,
        formula
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_symbol(pool: &PgPool, id: Uuid) -> sqlx::Result<Symbol> {
    sqlx::query_as!(
        Symbol,
        "
DELETE FROM wisdom.symbols
WHERE id = $1
RETURNING id, title, formula, created_at, updated_at
        ",
        id
    )
    .fetch_one(pool)
    .await
}
