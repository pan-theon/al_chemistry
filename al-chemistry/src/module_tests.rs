use crate::periodic_table::PeriodicTable;
use crate::matter::{element::*, substance::*};

#[test]
fn periodic_table_get() {
    let elements = ["C", "N", "Fm", "Mg"];
    let p_t = PeriodicTable::new();
    for name in elements {
        let el = p_t.get(name).unwrap();
        assert!(el.charge > 0);
    }

    let not_elements = ["?", "Ku", "Noe", "Arm"];
    for name in not_elements {
        assert!(p_t.get(name).is_none());
    }
}

#[test]
fn periodic_table_remove() {
    let mut p_t = PeriodicTable::new();
    let elements = ["Ti", "V", "Cr"];
    let mut i = 22;
    for name in elements {
        let (n, el) = p_t.remove(name).unwrap();
        assert!(n == name.to_string());
        assert!(el.charge == i);
        i += 1;
    }
}

#[test]
fn periodic_table_add() {
    let mut p_t = PeriodicTable::new();
    let not_elements = ["Ku", "Noe", "Arm"];
    let el = p_t.get(&"O").unwrap().clone();

    p_t.insert("?".to_string(), el.clone());
    assert!(p_t.get(&"?").is_none());

    p_t.insert("Fm".to_string(), el.clone());
    assert!(p_t.get(&"Fm").unwrap().charge != el.charge);

    for i in 0..not_elements.len() {
        p_t.insert(not_elements[i].to_string(),
                    Element {
                        charge: 101 + i as u16,
                        ..el.clone()
                    }
        );
    }
    for name in not_elements {
        assert!(p_t.get(name).is_some());
    }
}

#[test]
fn substance_simple() {
    let p_t = PeriodicTable::new();
    let mes = ["K", "Al", "Ti", "Sm"];
    let anti_mes = ["O", "Cl", "S", "Xe"];
    let not_elements = ["Pm O", "op", "Pol"];

    for m in mes {
        let s = Substance::from_string(m, &p_t).unwrap();
        assert!(s.class == SubstanceClass::Simple);
        assert!(s.me.len() == 1);
        assert!(s.me[m].index == 1);
        assert!(s.me[m].oxidation_state == 0);
        assert!(s.anti_me.len() == 0);
    }
    for a in anti_mes {
        let s = Substance::from_string(a, &p_t).unwrap();
        assert!(s.class == SubstanceClass::Simple);
        assert!(s.anti_me.len() == 1);
        assert!(s.anti_me[a].index == 1);
        assert!(s.anti_me[a].oxidation_state == 0);
        assert!(s.me.len() == 0);
    } 
    for name in not_elements {
        assert!(Substance::from_string(name, &p_t).is_err());
    }
}

#[test]
fn substance_oxide() {
    let o = vec!["SiO2", "CO", "CO2", "H2O"];
    let n_o = vec!["Al(OH)3", "HClO", "Na2O2"];
    let n = vec!["Si2O", "XeO"];

    is_substance_class(o, n_o, n, SubstanceClass::Oxide, vec![("O", -2)]);
}

#[test]
fn substance_peroxide() {
    let p = vec!["Na2O2", "MgO2", "SrO2"];
    let n_p = vec!["CaO", "RaO", "Li2O"];
    let n = vec!["Ca3O2", "CoO2", "MoO5"];

    is_substance_class(p, n_p, n, SubstanceClass::Peroxide, vec![("O", -1)]);
}

#[test]
fn substance_hydride() {
    let h = vec!["NaH", "CaH2", "SiH2", "OsH3"];
    let n_h = vec!["HCl", "HI", "H2O"];
    let n = vec!["SiH", "XeH"];

    is_substance_class(h, n_h, n, SubstanceClass::Hydride, vec![("H", -1)]);
}

#[test]
fn substance_base() {
    let b = vec!["Al(OH)3", "NaOH", "H2BeO2"];
    let n_b = vec!["B(OH)3", "H2SO4", "HClO"];
    let n = vec!["AlOH2", "Na(OH)3", "KH2O"];

    is_substance_class(b, n_b, n, SubstanceClass::Base, vec![("O", -2), ("H", 1)]);
}

#[test]
fn substance_acid() {
    let a = vec!["B(OH)3", "HCl", "H2SiO3", "H3PO4"];
    let n_a = vec!["H4TiO4", "H2CaO2", "H2O"];
    let n = vec!["HPO4", "H3Cl", "H2ClO"];

    is_substance_class(a, n_a, n, SubstanceClass::Acid, vec![("H", 1)]);
}

#[test]
fn substance_salt() {
    let s = vec!["NaCl", "NaHCO3", "Al(OH)CO3", "Al(OH)2I", "LiKRbPO4", "CsAuCl4"];
    let n_s = vec!["HCl", "Na(OH)"];
    let n = vec!["NaCl2", "CaPO4", "B5P3"];

    is_substance_class(s, n_s, n, SubstanceClass::Salt, vec![]);
}

fn is_substance_class(
    g_class: Vec<&str>, // g stands for "group"
    g_not_class: Vec<&str>,
    g_not_sb: Vec<&str>,
    class: SubstanceClass,
    imp_anti_me: Vec<(&str, i8)>
    )
{
    let p_t = PeriodicTable::new();
    for c in g_class {
        let c = Substance::from_string(c, &p_t).unwrap();
        println!("{}", c);
        assert!(c.class == class);
        for i in &imp_anti_me {
            assert!(c.anti_me[i.0].oxidation_state == i.1);
        }
    }

    for n_c in g_not_class {
        println!("{:#?}", Substance::from_string(n_c, &p_t).unwrap());
        assert!(Substance::from_string(n_c, &p_t).unwrap().class != class);
    }
    
    for n in g_not_sb {
        assert!(Substance::from_string(n, &p_t).is_err());
    }
}
