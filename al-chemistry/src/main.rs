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

    let reaction = Reaction::try_calculate_from(
        vec![
            Substance::from_string("Li", &periodic_table).unwrap(),
            Substance::from_string("O2", &periodic_table).unwrap(),
        ],
        false,
    )
    .unwrap();
    dbg!(reaction.products);

    let reaction = Reaction::try_calculate_from(
        vec![
            Substance::from_string("Cr", &periodic_table).unwrap(),
            Substance::from_string("S", &periodic_table).unwrap(),
        ],
        true,
    )
    .unwrap();
    dbg!(reaction.products);

    let reaction = Reaction::try_calculate_from(
        vec![
            Substance::from_string("Fe", &periodic_table).unwrap(),
            Substance::from_string("O2", &periodic_table).unwrap(),
        ],
        true,
    )
    .unwrap();
    dbg!(reaction.products);

    let test = Substance::from_string("Na2O2", &periodic_table);
    dbg!(test);
    let reaction = Reaction::try_calculate_from(
        vec![
            Substance::from_string("Na", &periodic_table).unwrap(),
            Substance::from_string("O2", &periodic_table).unwrap(),
        ],
        true,
    )
    .unwrap();
    dbg!(reaction.products);

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
