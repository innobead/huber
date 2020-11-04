use std::fmt::Formatter;

use serde::export::fmt;
use serde::export::fmt::Display;
use serde::{Deserialize, Serialize};

use crate::model::package::Package;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseIndex {
    pub name: String,
    pub version: String,
    pub owner: String,
    pub source: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    pub package: Package,
    pub version: String,
    pub is_current: bool,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (version: {}, source: {})",
            self.package.name,
            self.version,
            self.package.source.to_string()
        )
    }
}
