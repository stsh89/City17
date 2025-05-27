use crate::support;
use catacombs17_storage::wisdom::{NewSymbolRecord, create_symbol};
use sqlx::PgPool;

#[sqlx::test]
async fn it_creates_symbol(pool: PgPool) -> sqlx::Result<()> {
    let result = create_symbol(
        &pool,
        NewSymbolRecord {
            title: "circle".to_string(),
            formula: "x² + y² = r²".to_string(),
        },
    )
    .await;

    assert!(result.is_ok());

    let count = support::count_symbols(&pool).await?;

    assert_eq!(count, 1);

    Ok(())
}

#[sqlx::test]
async fn it_rejects_duplicate_symbols(pool: PgPool) -> sqlx::Result<()> {
    create_symbol(
        &pool,
        NewSymbolRecord {
            title: "circle".to_string(),
            formula: "x² + y² = r²".to_string(),
        },
    )
    .await?;

    let result = create_symbol(
        &pool,
        NewSymbolRecord {
            title: "circle".to_string(),
            formula: "x² + y² = r²".to_string(),
        },
    )
    .await;

    match result {
        Err(sqlx::Error::Database(err)) => assert!(err.is_unique_violation()),
        _ => unreachable!(),
    }

    Ok(())
}
