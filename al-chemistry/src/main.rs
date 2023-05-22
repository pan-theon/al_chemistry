mod matter;

mod periodic_table;
use periodic_table::PeriodicTable;

mod parser;

mod reaction;
use reaction::Reaction;

mod math_util;

fn main() {
    // periodic table example
    let periodic_table = PeriodicTable::new().unwrap();
    println!("{}", periodic_table.get("Cr").unwrap().a_rm);

    use crate::matter::substance::*;

    let alohco3 = Substance::from_string(&"Al(OH)CO3", &periodic_table).unwrap();
    assert!(alohco3.class == SubstanceClass::Salt);
    assert!(alohco3.content.get("OH").is_some());

    let t = Substance::from_string(&"NaAlO", &periodic_table).unwrap();
    assert!(t.class == SubstanceClass::Salt);

    let t2 = Substance::from_string(&"Al(OH)2Cl", &periodic_table).unwrap();
    assert!(t.class == SubstanceClass::Salt);

    let reaction = Reaction::try_calculate_from(vec![
        Substance::from_string("Li", &periodic_table).unwrap(),
        Substance::from_string("O2", &periodic_table).unwrap(),
    ]);
    dbg!(reaction);

    let reaction = Reaction::try_calculate_from(vec![
        Substance::from_string("Al", &periodic_table).unwrap(),
        Substance::from_string("O2", &periodic_table).unwrap(),
    ]);
    dbg!(reaction);

    let reaction = Reaction::try_calculate_from(vec![
        Substance::from_string("Zn", &periodic_table).unwrap(),
        Substance::from_string("O2", &periodic_table).unwrap(),
    ]);
    dbg!(reaction);
}
