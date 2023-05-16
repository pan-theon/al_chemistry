use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::matter::element::Element;

extern crate serde;
extern crate serde_json;

const DEFAULT_PERIODIC_TABLE_PATH: &str = ".applications/periodic_table.json";

#[derive(Deserialize)]
pub struct PeriodicTable {
    table: HashMap<String, Element>,
}

impl PeriodicTable {
    pub fn new() -> Result<PeriodicTable, Box<dyn std::error::Error>> {
        read_periodic_table_from_file(DEFAULT_PERIODIC_TABLE_PATH)
    }

    pub fn from(path: &str) -> Result<PeriodicTable, Box<dyn std::error::Error>> {
        read_periodic_table_from_file(path)
    }

    pub fn get(&self, key: &str) -> Option<&Element> {
        self.table.get(key)
    }
}

// The error trait is used because the function can revert 2 different types of errors (fs error and serde error)
fn read_periodic_table_from_file(path: &str) -> Result<PeriodicTable, Box<dyn std::error::Error>> {
    let table = fs::read_to_string(path)?;
    let table = serde_json::from_str(&table)?;
    Ok(PeriodicTable { table })
}
