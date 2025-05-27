use crate::{QueryPage, QueryPageToken};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub enum ListSymbolsParameters {
    NextPage(QueryPageToken),
    FirstPage(usize),
}

pub struct SymbolRecord {
    pub id: Uuid,
    pub title: String,
    pub formula: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SymbolRecordChanges {
    pub title: Option<String>,
    pub formula: Option<String>,
}

pub struct NewSymbolRecord {
    pub title: String,
    pub formula: String,
}

pub async fn create_symbol(pool: &PgPool, symbol: NewSymbolRecord) -> sqlx::Result<SymbolRecord> {
    let NewSymbolRecord { title, formula } = symbol;

    sqlx::query_as!(
        SymbolRecord,
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
) -> sqlx::Result<QueryPage<SymbolRecord>> {
    struct Filter {
        limit: usize,
        id: Option<Uuid>,
    }

    let filter = match parameters {
        ListSymbolsParameters::NextPage(QueryPageToken {
            id,
            limit,
            has_more,
        }) => {
            if !has_more {
                return Ok(QueryPage {
                    items: vec![],
                    next_page_token: None,
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
        SymbolRecord,
        "
SELECT id, title, formula, created_at, updated_at
FROM wisdom.symbols
WHERE case when $1::uuid is null then true else id > $1 end
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
        Some(QueryPageToken {
            id: items.last().unwrap().id,
            limit,
            has_more,
        })
    } else {
        None
    };

    Ok(QueryPage {
        items,
        next_page_token: token,
    })
}

pub async fn update_symbol(
    pool: &PgPool,
    id: Uuid,
    changes: SymbolRecordChanges,
) -> sqlx::Result<bool> {
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
