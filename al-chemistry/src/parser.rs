use crate::{matter::element::Element, periodic_table::PeriodicTable};
use std::collections::HashMap;

// no regex, as other not-really-needed third-party crates
pub fn collect_elements(
    reagents: &str,
    periodic_table: &PeriodicTable,
) -> Result<Vec<HashMap<String, (Element, u8)>>, &'static str> {
    let mut substances = Vec::<HashMap<String, (Element, u8)>>::new();
    let mut substance = HashMap::<String, (Element, u8)>::new();

    let mut element = String::new();
    let mut index = String::new();

    let reagents = format!("{} ", reagents);
    let mut chars = reagents.chars();

    let mut group = Vec::<String>::new();
    let mut is_closed = false;
    let mut check = Vec::<u8>::new();
    while let Some(c) = chars.next() {
        if c >= '0' && c <= '9' || c >= '₀' && c <= '₉' {
            index = format!("{}{}", index, c);
            continue;
        }
        if is_closed {
            while let Some(element) = group.pop() {
                match substance.get_mut(&element) {
                    Some(mut e) => e.1 *= index.parse::<u8>().unwrap_or(1),
                    None => return Err(&"There's an error while parsing reagents"),
                };
            }
            index = String::new();
            is_closed = false;
        }
        if c >= 'a' && c <= 'z' && element.len() > 0 {
            element = format!("{}{}", element, c);
            continue;
        }
        match periodic_table.get(&element) {
            Some(e) => {
                let i = index.parse::<u8>().unwrap_or(1);
                index = String::new();
                match substance.get_mut(&element) {
                    Some(e) => e.1 += i,
                    None => {
                        substance.insert(element.clone(), (e.clone(), i));
                    }
                }

                if check.len() > 0 {
                    group.push(element);
                }
            }
            None => {
                if element.len() != 0 {
                    return Err(&"Unknown element in reagents");
                }
            }
        };
        if ['[', '{', '('].contains(&c) {
            check.push(1);
            element = String::new();
            continue;
        }
        if [']', '}', ')'].contains(&c) {
            if check.pop() == None {
                return Err(&"Unclosed bracket in reagents");
            }
            is_closed = true;
            element = String::new();
            continue;
        }
        if c >= 'A' && c <= 'Z' {
            element = String::from(c);
        } else {
            element = String::new();
            substances.push(substance);
            substance = HashMap::new();
        }
    }

    if check.len() > 0 {
        return Err(&"Unbalanced brackets in reagents");
    }
    match substances.is_empty() {
        true => Err(&"String of reagents is incosistent - nothing recognized"),
        false => Ok(substances),
    }
}
