use super::{OperationalError, Page, PageToken, Symbol};

pub trait ListSymbols {
    async fn list_symbols(
        &self,
        next_page_token: Option<PageToken>,
    ) -> Result<Page<Symbol>, OperationalError>;
}

pub struct ListSymbolsOperation<'a, R> {
    pub repo: &'a R,
}

impl<'a, R> ListSymbolsOperation<'a, R>
where
    R: ListSymbols,
{
    pub async fn execute(
        &self,
        next_page_token: Option<PageToken>,
    ) -> Result<Page<Symbol>, OperationalError> {
        self.repo.list_symbols(next_page_token).await
    }
}
