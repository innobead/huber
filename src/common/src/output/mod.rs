use std::io::Write;
use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::output::console::ConsoleOutput;

pub mod console;

pub enum OutputFormat {
    Console,
    Json,
    Yaml,
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

pub trait OutputTrait {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        include_keys: Option<Vec<&str>>,
        exclude_keys: Option<Vec<&str>>,
    ) -> Result<()>;
}

pub fn new(format: &OutputFormat) -> impl OutputTrait {
    match format {
        _ => ConsoleOutput::new(),
    }
}
