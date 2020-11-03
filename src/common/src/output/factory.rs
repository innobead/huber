use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::output::console::ConsoleOutput;
use crate::output::json::JsonOutput;
use crate::output::yaml::YamlOutput;
use crate::output::{OutputFormat, OutputTrait};
use crate::result::Result;

pub struct FactoryConsole {
    pub output: OutputFormat,
}

impl FactoryConsole {
    pub fn new(output: OutputFormat) -> Self {
        Self { output }
    }
}

impl OutputTrait for FactoryConsole {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        include_keys: Option<Vec<&str>>,
        exclude_keys: Option<Vec<&str>>,
    ) -> Result<()> {
        match self.output {
            OutputFormat::Console => {
                ConsoleOutput::new().display(writer, obj, include_keys, exclude_keys)
            }

            OutputFormat::Yaml => {
                YamlOutput::new().display(writer, obj, include_keys, exclude_keys)
            }

            OutputFormat::Json => {
                JsonOutput::new().display(writer, obj, include_keys, exclude_keys)
            }
        }
    }
}
