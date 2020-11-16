use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "{:?}", self)
    }
}
