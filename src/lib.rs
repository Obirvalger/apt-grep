use std::collections::BTreeSet;

pub mod search_engine;
mod search_query;
pub use search_query::SearchQuery;

pub type StrSet<'a> = BTreeSet<&'a str>;
