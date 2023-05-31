use std::collections::HashMap;

use crate::{
    math_util::lcm,
    matter::{
        element::Element,
        substance::{Substance, SubstanceBlock, SubstanceClass as SC},
    },
    periodic_table::PeriodicTable,
};

// Electrochmical series of metalls. Where:
// from left to right the standard electrochemical potential increases
const ESMETALLS: [&str; 29] = [
    "Li", "Cs", "Rb", "K", "Ba", "Sr", "Ca", "Na", "Mg", "Al", "Ti", "Mn", "Zn", "Cr", "Fe", "Cd",
    "Co", "Ni", "Sn", "Pb", "H", "Sb", "Bi", "Cu", "Hg", "Ag", "Pd", "Pt", "Au",
];

#[derive(Debug)]
pub enum ReactionClass {
    MetallOxygen, // result: metall oxyde
    Unknown,
}

#[derive(Debug)]
pub enum Condition {}

#[derive(Debug)]
pub struct Reaction {
    pub reagents: Vec<Substance>,
    pub products: Vec<Substance>,
    pub class: ReactionClass,
}

impl Reaction {
    pub fn try_calculate_from(reagents: Vec<Substance>) -> Self {
        if reagents.len() != 2 {
            panic!("Supports reaction simulation only for 2 reagents")
        }

        let p_t = PeriodicTable::new();
        let reaction_class = Self::determine_class(&reagents, &p_t);
        let products = match reaction_class {
            ReactionClass::MetallOxygen => Self::metall_oxygen_reaction(reagents.clone(), &p_t),
            ReactionClass::Unknown => panic!("Unknown reaction"),
        };

        let mut unwrapped_products = Vec::new();
        for product in products {
            unwrapped_products.push(product.unwrap())
        }

        Self {
            reagents: reagents,
            products: unwrapped_products,
            class: reaction_class,
        }
    }

    fn determine_class(reagents: &Vec<Substance>, p_t: &PeriodicTable) -> ReactionClass {
        // We can determine reaction class by this substances
        let oxygen = Substance::from_string("O2", p_t).unwrap();

        // Characteristic of reagents
        let mut reagent_classes = Vec::<SC>::new(); // SubstanceClasses of reagents
        let mut contains_oxyde = false;
        let mut contains_metall = false;

        // Fill characteristics of reagents
        for substance in reagents {
            reagent_classes.push(substance.class); // fill reagent class into vector

            if substance.anti_me.len() == 0 && substance.me.len() > 0 {
                contains_metall = true;
            }

            if substance.eq(&oxygen) {
                // reaction contains oxygen
                contains_oxyde = true;
            }
        }

        // Me + O2 (Metall and Oxyde)
        if reagent_classes.contains(&SC::Simple) && contains_oxyde && contains_metall {
            return ReactionClass::MetallOxygen;
        }

        ReactionClass::Unknown
    }

    // result: metall oxyde
    fn metall_oxygen_reaction(
        reagents: Vec<Substance>,
        p_t: &PeriodicTable,
    ) -> Vec<Result<Substance, &'static str>> {
        // Get metall information
        let (metall_name, metall_element) = get_simple_metall_from_reagents(&reagents).unwrap();

        // Construct products
        // TODO: Add exceptions to the rules (example: Na + O2 = Na2O2 peroxyde)
        let (metall_index, oxygen_index) = calculate_indexes_for_2(*metall_element.valencies.last().unwrap(), 2u8);

        let oxygen_element = p_t.get("O").unwrap().clone();

        let mut map: HashMap<String, SubstanceBlock> = HashMap::new();
        map.insert(
            metall_name,
            SubstanceBlock::new(metall_element, metall_index, 0),
        );
        map.insert(
            "O".to_string(),
            SubstanceBlock::new(oxygen_element, oxygen_index, 0),
        );

        vec![Substance::from_elements(map)]
    }
}

fn get_simple_metall_from_reagents(reagents: &Vec<Substance>) -> Option<(String, Element)> {
    for reagent in reagents.iter() {
        if reagent.me.len() == 1 && reagent.anti_me.len() == 0 {
            let reagent_iter = reagent.me.iter().next().unwrap();
            return Some((reagent_iter.0.clone(), reagent_iter.1.element.clone()));
        }
    }
    None
}

// Calculation of indexes for two elements.
// Idea: element_index = LCM(first_valence, second_valence) / element_oxydation
fn calculate_indexes_for_2(first_valence: u8, second_valence: u8) -> (u8, u8) {
    let oxyd_lcm = lcm(first_valence, second_valence);
    (oxyd_lcm / first_valence, oxyd_lcm / second_valence)
}
