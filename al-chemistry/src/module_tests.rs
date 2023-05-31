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
