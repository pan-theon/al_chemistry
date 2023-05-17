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
