use std::fmt::Display;
use serde::export::Formatter;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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