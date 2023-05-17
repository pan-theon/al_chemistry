use std::collections::HashMap;

mod matter;
use matter::element::Element;

mod periodic_table;
use periodic_table::PeriodicTable;

mod parser;

fn main() {
    // periodic table example
    let periodic_table = PeriodicTable::new().unwrap();
    println!("{}", periodic_table.get("Cr").unwrap().a_rm);

    use crate::matter::substance::*;

    let alohco3 = Substance::from_string(&"Al(OH)CO3", &periodic_table).unwrap();
    assert!(alohco3.class == SubstanceClass::Salt);
    assert!(alohco3.content.get("OH").is_some());
}

fn get_periodic_table() -> Result<HashMap<String, Element>, &'static str> {
    extern crate serde;
    extern crate serde_json;

    use std::fs;

    let table_file = fs::read_to_string(".applications/periodic_table.json");
    let table: String;
    match table_file {
        Err(_) => return Err(&"Unable to open file '.applications/periodic_table.json'"),
        Ok(t) => table = t,
    };

    let table = serde_json::from_str(&table);
    match table {
        Err(_) => Err(&"Periodic table is inconsistent
            ('.applications/periodic_table.json')"),
        Ok(res) => Ok(res),
    }
}
