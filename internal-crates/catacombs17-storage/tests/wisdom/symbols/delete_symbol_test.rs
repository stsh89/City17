use crate::support;
use catacombs17_storage::wisdom::delete_symbol;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("symbols"))]
async fn it_deletes_symbol(pool: PgPool) -> sqlx::Result<()> {
    let count_was = support::count_symbols(&pool).await?;

    let res = delete_symbol(
        &pool,
        Uuid::try_parse("0197031d-b60c-7f60-9084-67b2a761bafb").unwrap(),
    )
    .await;

    assert!(res.is_ok());

    let count = support::count_symbols(&pool).await?;

    assert_eq!(count_was - count, 1);

    Ok(())
}

#[sqlx::test]
async fn it_fails_if_symbol_does_not_exist(pool: PgPool) -> sqlx::Result<()> {
    let res = delete_symbol(
        &pool,
        Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap(),
    )
    .await;

    assert!(matches!(res, Err(sqlx::Error::RowNotFound)));

    Ok(())
}
