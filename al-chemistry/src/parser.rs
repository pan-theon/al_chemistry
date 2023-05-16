use crate::{matter::element::Element, periodic_table::PeriodicTable};
use std::collections::HashMap;

// no regex, as other not-really-needed third-party crates
pub fn collect_elements(
    reagents: &str,
    periodic_table: &PeriodicTable,
) -> Result<Vec<HashMap<String, (Element, u8)>>, &'static str> {
    let mut substances = Vec::<HashMap<String, (Element, u8)>>::new();
    let mut substance = HashMap::<String, (Element, u8)>::new();

    let mut element: String;
    let mut index: String;

    let mut chars_iter = reagents.chars();
    let mut c = chars_iter.next();
    while c != None {
        // collect element in substance...
        element = c.unwrap().to_string();
        c = chars_iter.next();
        while c != None && c > Some('a') && c < Some('z') {
            element += &c.unwrap().to_string();
            c = chars_iter.next();
        }
        let el = match periodic_table.get(&element) {
            None => return Err(&"There's unknown element in reagents"),
            Some(e) => e.clone(),
        };

        // with it's index
        index = String::from("");
        while c != None && c > Some('0') && c < Some('9') {
            index += &c.unwrap().to_string();
            c = chars_iter.next();
        }
        let i = index.parse::<u8>().unwrap_or(1);

        if let Some(v) = substance.get_mut(&element) {
            v.1 += i;
        } else {
            substance.insert(element, (el, i));
        }

        if c < Some('A') || c > Some('Z') || c == None {
            substances.push(substance);
            substance = HashMap::<String, (Element, u8)>::new();

            c = chars_iter.next();
            while c != None && (c < Some('A') || c > Some('Z')) {
                c = chars_iter.next();
            }
        }
    }
    match substances.is_empty() {
        true => Err(&"String of reagents is incosistent - nothing recognized"),
        false => Ok(substances),
    }
}
