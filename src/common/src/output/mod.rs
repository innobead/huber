use std::io::Write;
use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod console;
pub mod factory;
pub mod json;
pub mod yaml;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum OutputFormat {
    Console,
    Yaml,
    Json,
}

pub trait OutputTrait {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        include_keys: Option<Vec<&str>>,
        exclude_keys: Option<Vec<&str>>,
    ) -> Result<()>;
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "console" => Ok(OutputFormat::Console),
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("{} not supported format", s)),
        }
    }
}
