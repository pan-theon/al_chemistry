use std::collections::HashMap;

mod matter;
use matter::element::Element;

mod parser;
use parser::parse;

fn main() {
    let periodic_table: HashMap<String, Element> = match get_periodic_table() {
        Err(e) => { println!("{e}"); return; },
        Ok(t) => t,
    };

    let ss = &parse(String::from("H2SO4 + AlOHCO3, AgCl"), &periodic_table).unwrap();
    use crate::matter::substance::SubstanceClass;
    assert!(ss[0].class == SubstanceClass::Acid);
    assert!(ss[1].class == SubstanceClass::Salt);
    assert!(ss[2].class == SubstanceClass::Salt);
    assert!(ss[1].content.get("OH").is_some());
}

fn get_periodic_table() -> Result<HashMap<String, Element>, &'static str> {
    extern crate serde;
    extern crate serde_json;

    use std::fs;

    let table_file = fs::read_to_string(".applications/periodic_table.json");
    let table: String;
    match table_file {
        Err(_) => return
            Err(&"Unable to open file '.applications/periodic_table.json'"),
        Ok(t) => table = t,
    };

    let table = serde_json::from_str(&table);
    match table {
        Err(_) => Err(&"Periodic table is inconsistent
            ('.applications/periodic_table.json')"),
        Ok(res) => Ok(res),
    }
}
