use crate::operations::{
    ListSymbols, OperationalError, Page, PageToken, Symbol, SymbolAttributes, SymbolFormula,
    SymbolTitle,
};
use catacombs17_storage::{
    QueryPage, QueryPageToken,
    wisdom::{self, ListSymbolsParameters, SymbolRecord},
};
use sqlx::PgPool;

const DEFAULT_PAGE_LIMIT: usize = 3;

pub struct Repo {
    pool: PgPool,
}

impl Repo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ListSymbols for Repo {
    async fn list_symbols(
        &self,
        next_page_token: Option<PageToken>,
    ) -> Result<Page<Symbol>, OperationalError> {
        let parameters = match next_page_token {
            Some(page_token) => {
                let PageToken {
                    id,
                    limit,
                    has_more,
                } = page_token;

                ListSymbolsParameters::NextPage(QueryPageToken {
                    id,
                    limit,
                    has_more,
                })
            }
            None => ListSymbolsParameters::FirstPage(DEFAULT_PAGE_LIMIT),
        };

        let page = wisdom::list_symbols(&self.pool, parameters).await?;

        let QueryPage {
            items,
            next_page_token,
        } = page;

        Ok(Page {
            items: items
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            next_page_token: next_page_token.map(Into::into),
        })
    }
}

impl From<QueryPageToken> for PageToken {
    fn from(value: QueryPageToken) -> Self {
        let QueryPageToken {
            id,
            limit,
            has_more,
        } = value;

        Self {
            id,
            limit,
            has_more,
        }
    }
}

impl TryFrom<SymbolRecord> for Symbol {
    type Error = OperationalError;

    fn try_from(value: SymbolRecord) -> Result<Self, Self::Error> {
        let SymbolRecord {
            id,
            title,
            formula,
            created_at,
            updated_at,
        } = value;

        Ok(Symbol::new(SymbolAttributes {
            id,
            title: SymbolTitle::new(title)?,
            formula: SymbolFormula::new(formula)?,
            created_at,
            updated_at,
        }))
    }
}
