use std::collections::HashMap;

use crate::{
    math_util::lcm,
    matter::{
        element::Element,
        substance::{Substance, SubstanceBlock as SB, SubstanceClass as SC},
    },
    periodic_table::PeriodicTable,
};

static NO_REACTION: &str = "No reaction";

// Electrochmical series of metalls. Where:
// from left to right the standard electrochemical potential increases
const ESMETALLS: [&str; 29] = [
    "Li", "Cs", "Rb", "K", "Ba", "Sr", "Ca", "Na", "Mg", "Al", "Ti", "Mn", "Zn", "Cr", "Fe", "Cd",
    "Co", "Ni", "Sn", "Pb", "H", "Sb", "Bi", "Cu", "Hg", "Ag", "Pd", "Pt", "Au",
];

#[derive(Debug)]
pub enum ReactionType {
    Combination,
    Decomposition,
    Exchange,
    Substition,
}

#[derive(Debug)]
pub struct Reaction {
    pub reagents: Vec<Substance>,
    pub heating: bool,
    pub products: Vec<Substance>,
    pub rtype: ReactionType,
}

impl Reaction {
    pub fn try_calculate_from(
        reagents: Vec<Substance>,
        heating: bool,
    ) -> Result<Self, &'static str> {
        if reagents.len() != 2 {
            panic!("Supports reaction simulation only for 2 reagents")
        }

        let p_t = PeriodicTable::new();
        let reaction_func = match Self::determine_class(&reagents) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let (products, rtype) = match reaction_func(&reagents, heating, &p_t) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(Self {
            reagents: reagents,
            heating,
            products: products,
            rtype,
        })
    }

    fn determine_class(
        reagents: &Vec<Substance>,
    ) -> Result<
        fn(
            &Vec<Substance>,
            bool,
            &PeriodicTable,
        ) -> Result<(Vec<Substance>, ReactionType), &'static str>,
        &'static str,
    > {
        // Classes of reagents
        let mut reagent_classes = HashMap::from([
            (SC::Simple, 0),
            (SC::Hydride, 0),
            (SC::Oxide, 0),
            (SC::Peroxide, 0),
            (SC::Base, 0),
            (SC::Acid, 0),
            (SC::Salt, 0),
        ]);

        // Fill characteristics of reagents
        for substance in reagents {
            *reagent_classes.get_mut(&substance.class).unwrap() += 1;
        }

        if reagents.len() == 2 && reagent_classes[&SC::Simple] == 2 {
            Ok(Self::reaction_me_antime)
        } else {
            Err(&"Unknown class of reaction")
        }
    }

    fn reaction_me_antime(
        reagents: &Vec<Substance>,
        heating: bool,
        p_t: &PeriodicTable,
    ) -> Result<(Vec<Substance>, ReactionType), &'static str> {
        let rtype = ReactionType::Combination;
        let (me_name, me_element) = get_simple_me_from_reagents(reagents).unwrap();
        let (ame_name, ame_element) = get_simple_antime_from_reagents(reagents).unwrap();

        // Exceptions to the rules
        match (ame_element.charge, me_element.charge) {
            (8, 47 | 78 | 79) => return Err(NO_REACTION), // Oxyd and Ag Pt Au
            _ => (),
        }

        // Try to guess oxydation of metall.
        let me_oxydation = match (me_element.group, heating) {
            (1..=2, _) => me_element.group as i8, // needs little energy to give max electrons
            (3..=5, true) => me_element.group as i8, // statistics + logic
            (6, true) => me_element.valencies[(me_element.valencies.len() - 1) / 2] as i8, // statistics
            (_, true) => *me_element.valencies.last().unwrap() as i8,                      // xd
            (_, _) => return Err(NO_REACTION),
        };

        // Try to guess oxydation of anti metall
        let ame_oxydation = (18 - ame_element.group) as i8 * -1;

        // Calculate indexes
        let (mut me_index, mut ame_index) = calculate_indexes_for_2(me_oxydation, ame_oxydation);

        // Exceptions to the rules
        match (ame_element.charge, me_element.charge) {
            (8, 11) => {
                // Na + O2 should be peroxyde
                me_index = 2;
                ame_index = 2;
            }
            _ => (),
        }

        let mut map = HashMap::new();
        map.insert(me_name, SB::new(me_element, me_index, 0));
        map.insert(ame_name, SB::new(ame_element, ame_index, 0));
        let substance = match Substance::from_elements(map) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok((vec![substance], rtype))
    }
}

fn get_simple_me_from_reagents(reagents: &Vec<Substance>) -> Option<(String, Element)> {
    for reagent in reagents {
        if reagent.me.len() == 1 && reagent.anti_me.len() == 0 {
            let reagent_iter = reagent.me.iter().next().unwrap();
            return Some((reagent_iter.0.clone(), reagent_iter.1.element.clone()));
        }
    }
    None
}

fn get_simple_antime_from_reagents(reagents: &Vec<Substance>) -> Option<(String, Element)> {
    for reagent in reagents {
        if reagent.me.len() == 0 && reagent.anti_me.len() == 1 {
            let reagent_iter = reagent.anti_me.iter().next().unwrap();
            return Some((reagent_iter.0.clone(), reagent_iter.1.element.clone()));
        }
    }
    None
}

// Calculation of indexes for two elements.
// Idea: element_index = LCM(first_valence, second_valence) / element_oxydation
fn calculate_indexes_for_2(first_oxydation: i8, second_oxydation: i8) -> (u8, u8) {
    let first_oxydation = first_oxydation.abs() as u8;
    let second_oxydation = second_oxydation.abs() as u8;
    let lcm = lcm(first_oxydation, second_oxydation);
    (lcm / first_oxydation, lcm / second_oxydation)
}
