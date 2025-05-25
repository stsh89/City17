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

pub struct SymbolChanges {
    pub title: Option<String>,
    pub formula: Option<String>,
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

pub async fn delete_symbol(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let rows_affected = sqlx::query_as!(
        Symbol,
        "
DELETE FROM wisdom.symbols
WHERE id = $1
        ",
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn update_symbol(pool: &PgPool, id: Uuid, changes: SymbolChanges) -> sqlx::Result<bool> {
    let rows_affected = sqlx::query_as!(
        Symbol,
        "
UPDATE wisdom.symbols
SET
    title = coalesce($2, title),
    formula = coalesce($3, formula)
WHERE id = $1
        ",
        id,
        changes.title,
        changes.formula
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}
