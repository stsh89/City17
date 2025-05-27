use crate::{
    AppState,
    api::{PagePresenter, error::ApiError},
    operations::{ListSymbolsOperation, Symbol},
};
use chrono::{DateTime, Utc};
use rocket::{State, serde::json::Json};
use uuid::Uuid;

#[get("/?<next_page_token>")]
#[tracing::instrument(skip(state), name = "Symbols index", err(Debug))]
pub async fn index(
    state: &State<AppState>,
    next_page_token: Option<String>,
) -> Result<Json<PagePresenter<SymbolPresenter>>, ApiError> {
    let repo = &state.repo;

    let next_page_token = next_page_token.map(|token| token.parse()).transpose()?;

    let page = ListSymbolsOperation { repo }
        .execute(next_page_token)
        .await?;

    Ok(Json(PagePresenter {
        items: page.items.into_iter().map(Into::into).collect(),
        next_page_token: page.next_page_token.map(|token| token.to_string()),
    }))
}

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct SymbolPresenter {
    id: Uuid,
    title: String,
    formula: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Symbol> for SymbolPresenter {
    fn from(value: Symbol) -> Self {
        Self {
            id: value.id(),
            title: value.title().to_string(),
            formula: value.formula().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}
