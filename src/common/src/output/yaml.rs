use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::output::OutputTrait;
use crate::result::Result;

pub struct YamlOutput;

impl YamlOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputTrait for YamlOutput {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        _include_keys: Option<Vec<&str>>,
        _exclude_keys: Option<Vec<&str>>,
    ) -> Result<()> {
        Ok(serde_yaml::to_writer(writer, obj)?)
    }
}
