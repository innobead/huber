use std::any::Any;
use std::io::Write;

use anyhow::{Result};
use inflector::Inflector;
use prettytable::{format, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::output::OutputTrait;

pub struct ConsoleOutput;

impl OutputTrait for ConsoleOutput {
    fn display<'a, T: Deserialize<'a> + Serialize>(
        &self,
        writer: impl Write,
        obj: &T,
        include_keys: Option<Vec<&str>>,
        exclude_keys: Option<Vec<&str>>,
    ) -> Result<()> {
        let mut writer = writer;
        let obj = serde_json::to_value(obj)?;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);

        match obj {
            _ if obj.is_array() => {
                for x in obj.as_array().unwrap() {
                    self.display_obj(&mut table, x, &include_keys, &exclude_keys)?;
                }
            }

            _ if obj.is_object() => {
                self.display_obj(&mut table, &obj, &include_keys, &exclude_keys)?;
            }

            _ => Err(anyhow!("unsupported display type: {:?}", obj.type_id()))?,
        };

        table
            .print(&mut writer)
            .map(|_it| ())
            .map_err(|it| anyhow!(it))
    }
}

impl ConsoleOutput {
    pub fn new() -> Self {
        Self {}
    }

    fn display_obj(
        &self,
        table: &mut Table,
        obj: &Value,
        include_keys: &Option<Vec<&str>>,
        exclude_keys: &Option<Vec<&str>>,
    ) -> Result<()> {
        let obj = obj.as_object().unwrap();

        let predict_include_keys = |it: &(&String, &Value)| {
            if let Some(keys) = include_keys {
                keys.contains(&it.0.to_lowercase().as_str())
            } else {
                true
            }
        };

        let predict_exclude_keys = |it: &(&String, &Value)| {
            if let Some(keys) = exclude_keys {
                !keys.contains(&it.0.to_lowercase().as_str())
            } else {
                true
            }
        };

        if table.is_empty() {
            let columns = obj
                .iter()
                .filter(predict_include_keys)
                .filter(predict_exclude_keys)
                .map(|it| Cell::new(&it.0.to_title_case()))
                .collect();

            table.add_row(Row::new(columns));
        }

        let column_values = obj
            .iter()
            .filter(predict_include_keys)
            .filter(predict_exclude_keys)
            .map(|it| Cell::new(&to_string_trim(it.1)))
            .collect();

        table.add_row(Row::new(column_values));

        Ok(())
    }
}

fn to_string_trim(v: &Value) -> String {
    match v.to_string().as_str() {
        "null" => "".to_string(),
        _ => v
            .to_string()
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string(),
    }
}
