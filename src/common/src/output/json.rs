use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::output::OutputTrait;
use crate::result::Result;

pub struct JsonOutput;

impl JsonOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputTrait for JsonOutput {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        _include_keys: Option<Vec<&str>>,
        _exclude_keys: Option<Vec<&str>>,
    ) -> Result<()> {
        Ok(serde_json::to_writer_pretty(writer, obj)?)
    }
}
