mod matter;

mod periodic_table;
use periodic_table::PeriodicTable;

mod parser;

mod reaction;
use reaction::Reaction;

mod math_util;

fn main() {
    // periodic table example
    let periodic_table = PeriodicTable::new();
    println!("{}", periodic_table.get("Cr").unwrap().a_rm);

    use crate::matter::substance::*;

    let t = Substance::from_string("B(OH)3", &periodic_table);
    println!("{:#?}", t);

    /*
    let alohco3 = Substance::from_string(&"Al(OH)CO3", &periodic_table).unwrap();
    assert!(alohco3.class == SubstanceClass::Salt);
    assert!(alohco3.content.get("OH").is_some());

    let t = Substance::from_string(&"NaAlO2", &periodic_table).unwrap();
    assert!(t.class == SubstanceClass::Salt);

    let t = Substance::from_string(&"NaI", &periodic_table).unwrap();
    assert!(t.class == SubstanceClass::Salt);

    let t2 = Substance::from_string(&"Al(OH)2Cl", &periodic_table).unwrap();
    assert!(t2.class == SubstanceClass::Salt);
    */
}
