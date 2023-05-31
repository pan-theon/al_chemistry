use std::collections::HashMap;

use crate::{
    math_util::lcm,
    matter::{
        element::Element,
        substance::{Substance, SubstanceBlock, SubstanceClass as SC},
    },
    periodic_table::PeriodicTable,
};

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

        let reaction_class = Self::determine_class(&reagents);
        let products = match reaction_class {
            ReactionClass::MetallOxygen => Self::metall_oxygen_reaction(reagents.clone()),
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

    fn determine_class(reagents: &Vec<Substance>) -> ReactionClass {
        let p_t = PeriodicTable::new().unwrap();

        // We can determine reaction class by this substances
        let oxygen = Substance::from_string("O2", &p_t).unwrap();

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
        mut reagents: Vec<Substance>,
    ) -> Vec<Result<Substance, &'static str>> {
        let p_t = PeriodicTable::new().unwrap();

        // Get metall element from reagents
        for (i, reagent) in reagents.iter_mut().enumerate() {
            // remove O2 from reagents
            if let Some(_) = reagent.anti_me.remove_entry("O") {
                reagents.remove(i);
                break;
            }
        }

        let metall_substance = reagents.pop().unwrap(); // we can be sure that reagents contains only one Substance
        let metall_substance_iter = metall_substance.me.iter().next().unwrap();
        let metall_name = metall_substance_iter.0.clone();
        let metall_element = metall_substance_iter.1.element.clone();

        // Construct products
        // TODO: Add exceptions to the rules (example: Na + O2 = Na2O2 peroxyde)
        let (metall_index, oxygen_index): (u8, u8);
        let (metall_oxydation, oxygen_oxydation) = (*metall_element.valencies.last().unwrap(), 2u8); // yes, O2 has -2 oxydation state. +2 is used because it's easy to calculate

        // Calculation of indexes.
        // Idea: element_index = LCM / element_oxydation
        let oxyd_lcm = lcm(metall_oxydation, oxygen_oxydation);
        metall_index = oxyd_lcm / metall_oxydation;
        oxygen_index = oxyd_lcm / oxygen_oxydation;

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
