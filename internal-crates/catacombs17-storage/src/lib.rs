pub mod wisdom;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub struct QueryPage<T> {
    pub items: Vec<T>,
    pub next_page_token: Option<QueryPageToken>,
}

#[derive(Deserialize, Serialize)]
pub struct QueryPageToken {
    pub id: Uuid,
    pub limit: usize,
    pub has_more: bool,
}

impl QueryPageToken {
    pub fn has_more(&self) -> bool {
        self.has_more
    }
}

impl FromStr for QueryPageToken {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        serde_json::from_str(s).map_err(eyre::Error::new)
    }
}
