mod error;
mod list_symbols;

use std::{fmt::Display, str::FromStr};

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use error::OperationalError;
pub use list_symbols::*;

const MAX_SYMBOL_TITLE: usize = 100;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PageToken {
    pub id: Uuid,
    pub limit: usize,
    pub has_more: bool,
}

pub struct Page<T> {
    pub items: Vec<T>,
    pub next_page_token: Option<PageToken>,
}

pub struct Symbol {
    id: Uuid,
    title: SymbolTitle,
    formula: SymbolFormula,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct SymbolAttributes {
    pub id: Uuid,
    pub title: SymbolTitle,
    pub formula: SymbolFormula,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SymbolTitle(String);

pub struct SymbolFormula(String);

impl SymbolTitle {
    pub fn new(title: String) -> Result<Self, OperationalError> {
        let title = title.trim().to_string();

        if title.is_empty() {
            return Err(OperationalError::Validation(
                "symbol title cannot be empty".to_string(),
            ));
        }

        if title.len() > MAX_SYMBOL_TITLE {
            return Err(OperationalError::Validation(format!(
                "symbol title cannot be longer than {MAX_SYMBOL_TITLE} characters"
            )));
        }

        Ok(Self(title))
    }
}

impl SymbolFormula {
    pub fn new(formula: String) -> Result<Self, OperationalError> {
        let formula = formula.trim().to_string();

        if formula.is_empty() {
            return Err(OperationalError::Validation(
                "symbol formula cannot be empty".to_string(),
            ));
        }

        Ok(Self(formula))
    }
}

impl Display for PageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json_str = serde_json::to_string(self).map_err(|_err| std::fmt::Error)?;

        let encoded = URL_SAFE.encode(json_str.as_bytes());

        f.write_str(&encoded)
    }
}

impl FromStr for PageToken {
    type Err = OperationalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json_str = URL_SAFE.decode(s).map_err(|_err| {
            OperationalError::InvalidArgument("malformed page token".to_string())
        })?;

        serde_json::from_slice(&json_str).map_err(|_err| {
            OperationalError::InvalidArgument("malformed page token value".to_string())
        })
    }
}

impl Symbol {
    pub fn new(attributes: SymbolAttributes) -> Self {
        let SymbolAttributes {
            id,
            title,
            formula,
            created_at,
            updated_at,
        } = attributes;

        Self {
            id,
            title,
            formula,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &SymbolTitle {
        &self.title
    }

    pub fn formula(&self) -> &SymbolFormula {
        &self.formula
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Display for SymbolTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Display for SymbolFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
