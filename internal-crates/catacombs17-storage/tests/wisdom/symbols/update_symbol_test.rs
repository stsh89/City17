use catacombs17_storage::wisdom::{SymbolChanges, update_symbol};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("symbols"))]
async fn it_updates_symbol(pool: PgPool) -> sqlx::Result<()> {
    let is_updated = update_symbol(
        &pool,
        Uuid::try_parse("0197031d-b60c-7f60-9084-67b2a761bafb").unwrap(),
        SymbolChanges {
            title: Some("flower of life".to_string()),
            formula: Some("not yet discovered".to_string()),
        },
    );

    assert!(is_updated.await?);

    Ok(())
}

#[sqlx::test]
async fn it_returns_false_if_symbol_does_not_exist(pool: PgPool) -> sqlx::Result<()> {
    let is_updated = update_symbol(
        &pool,
        Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap(),
        SymbolChanges {
            title: Some("flower of life".to_string()),
            formula: Some("not yet discovered".to_string()),
        },
    );

    assert!(!is_updated.await?);

    Ok(())
}
