use crate::{Page, PageToken};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub enum ListSymbolsParameters {
    NextPage(PageToken),
    FirstPage(usize),
}

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

pub async fn list_symbols(
    pool: &PgPool,
    parameters: ListSymbolsParameters,
) -> sqlx::Result<Page<Symbol>> {
    struct Filter {
        limit: usize,
        id: Option<Uuid>,
    }

    let filter = match parameters {
        ListSymbolsParameters::NextPage(PageToken {
            id,
            limit,
            has_more,
        }) => {
            if has_more {
                return Ok(Page {
                    items: vec![],
                    token: None,
                });
            }

            Filter {
                limit,
                id: Some(id),
            }
        }
        ListSymbolsParameters::FirstPage(l) => Filter {
            limit: l + 1,
            id: None,
        },
    };

    let Filter { id, limit } = filter;

    let mut items = sqlx::query_as!(
        Symbol,
        "
SELECT id, title, formula, created_at, updated_at
FROM wisdom.symbols
WHERE case when $1::uuid is null then true else id > $1 end
ORDER BY id DESC
LIMIT $2
        ",
        id,
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    let has_more = items.len() == limit;

    if has_more {
        items.pop();
    }

    let token = if has_more {
        Some(PageToken {
            id: items.last().unwrap().id,
            limit,
            has_more,
        })
    } else {
        None
    };

    Ok(Page { items, token })
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
