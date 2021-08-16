use std::collections::BTreeSet;
use std::env;

pub mod search_engine;
mod search_query;
pub use search_query::SearchQuery;

pub type StrSet<'a> = BTreeSet<&'a str>;

pub fn getenv<K: AsRef<str>, D: AsRef<str>>(key: K, default: D) -> String {
    env::var(key.as_ref()).unwrap_or_else(|_| default.as_ref().to_string())
}

pub fn getenv_i64<K: AsRef<str>>(key: K, default: i64) -> i64 {
    match env::var(key.as_ref()) {
        Ok(val) => val.parse::<i64>().unwrap_or(default),
        Err(_e) => default,
    }
}
