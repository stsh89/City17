use catacombs17_storage::{
    QueryPage,
    wisdom::{ListSymbolsParameters, list_symbols},
};
use sqlx::PgPool;

#[sqlx::test(fixtures("symbols"))]
async fn it_lists_symbols(pool: PgPool) -> sqlx::Result<()> {
    let QueryPage {
        items: symbols,
        next_page_token,
    } = list_symbols(&pool, ListSymbolsParameters::FirstPage(3)).await?;

    assert_eq!(symbols.len(), 3);

    let names = symbols.iter().map(|s| &s.title).collect::<Vec<_>>();

    assert!(next_page_token.is_some());

    let next_page_token = next_page_token.unwrap();
    assert!(next_page_token.has_more());

    assert_eq!(names, vec!["circle", "triangle", "diamond"]);

    let page = list_symbols(&pool, ListSymbolsParameters::NextPage(next_page_token)).await?;

    let names = page.items.iter().map(|s| &s.title).collect::<Vec<_>>();
    assert_eq!(names, vec!["star", "square", "rectangle"]);

    Ok(())
}
