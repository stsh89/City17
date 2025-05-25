pub mod wisdom;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub struct Page<T> {
    pub items: Vec<T>,
    pub token: Option<PageToken>,
}

#[derive(Deserialize, Serialize)]
pub struct PageToken {
    id: Uuid,
    limit: usize,
    has_more: bool,
}

impl FromStr for PageToken {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        serde_json::from_str(s).map_err(eyre::Error::new)
    }
}
