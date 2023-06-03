use std::collections::HashMap;

use crate::{
    math_util::lcm,
    matter::{
        element::Element,
        substance::{Substance, SubstanceBlock as SB, SubstanceClass as SC},
    },
    periodic_table::PeriodicTable,
};

// Electrochmical series of metalls. Where:
// from left to right the standard electrochemical potential increases
const ACTIVE_METALLS: [&str; 8] = ["Li", "Cs", "Rb", "K", "Ba", "Sr", "Ca", "Na"];
const MEDIUM_ACTIVE_METALLS: [&str; 12] = [
    "Mg", "Al", "Ti", "Mn", "Zn", "Cr", "Fe", "Cd", "Co", "Ni", "Sn", "Pb",
];
const NON_ACTIVE_METALLS: [&str; 8] = ["Sb", "Bi", "Cu", "Hg", "Ag", "Pd", "Pt", "Au"];

#[derive(Debug, PartialEq)]
pub enum ReactionType {
    Combination,
    Decomposition,
    Exchange,
    Substition,
    None,
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
        let reaction_func = match Self::determine_class(&reagents, &p_t) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let (products, rtype) = match reaction_func(&reagents, heating, &p_t) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(Self {
            reagents,
            heating,
            products,
            rtype,
        })
    }

    fn determine_class(
        reagents: &Vec<Substance>,
        p_t: &PeriodicTable,
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

        // Some bools
        let mut contains_simple_me = false;
        let mut contains_simple_ame = false;
        let mut contains_water = false;

        // Some substances for comparing
        let water = Substance::from_string("H2O", &p_t).unwrap();

        // Fill characteristics of reagents
        for substance in reagents {
            *reagent_classes.get_mut(&substance.class).unwrap() += 1;

            match (substance.me.len(), substance.anti_me.len()) {
                (1, 0) => contains_simple_me = true,
                (0, 1) => contains_simple_ame = true,
                (_, _) => (),
            }

            if substance.eq(&water) {
                contains_water = true;
            }
        }

        if reagents.len() == 2 && contains_simple_me && contains_simple_ame {
            Ok(Self::reaction_me_antime)
        } else if reagents.len() == 2 && contains_simple_me && contains_water {
            Ok(Self::reaction_me_water)
        } else if reagents.len() == 2 && contains_simple_me && reagent_classes[&SC::Acid] == 1 {
            Ok(Self::reaction_me_acid)
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
        let (me_name, me_element) = get_simple_me_from_reagents(reagents);
        let (ame_name, ame_element) = get_simple_antime_from_reagents(reagents);

        // Exceptions to the rules
        match (ame_element.charge, me_element.charge) {
            (8, 47 | 78 | 79) => return Ok((vec![], ReactionType::None)), // Oxyd and Ag Pt Au
            _ => (),
        }

        // Try to guess oxydation of metall.
        let me_oxydation = match (me_element.group, heating) {
            (1..=2, _) => me_element.group as i8, // needs little energy to give max electrons
            (3..=5, true) => me_element.group as i8, // statistics + logic
            (6, true) => me_element.valencies[(me_element.valencies.len() - 1) / 2] as i8, // statistics
            (_, true) => *me_element.valencies.last().unwrap() as i8,                      // xd
            (_, false) => return Ok((vec![], ReactionType::None)),
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

    fn reaction_me_water(
        reagents: &Vec<Substance>,
        heating: bool,
        p_t: &PeriodicTable,
    ) -> Result<(Vec<Substance>, ReactionType), &'static str> {
        let rtype = ReactionType::Substition;
        let (me_name, me_element) = get_simple_me_from_reagents(reagents);

        // active_metall + water = base + H2
        if ACTIVE_METALLS.contains(&me_name.as_str()) {
            let base_oxydation = -1 as i8; // Base always has this oxydation
            let me_oxydation = me_element.group as i8;

            let (me_index, base_index) = calculate_indexes_for_2(me_oxydation, base_oxydation);

            let mut map = HashMap::new();
            map.insert(me_name, SB::new(me_element, me_index, 0));
            map.insert(
                "O".to_string(),
                SB::new(p_t.get("O").unwrap().clone(), base_index, 0),
            );
            map.insert(
                "H".to_string(),
                SB::new(p_t.get("H").unwrap().clone(), base_index, 0),
            );

            let base_substance = match Substance::from_elements(map) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let hydrogen = Substance::from_string("H2", p_t).unwrap();

            return Ok((vec![base_substance, hydrogen], rtype));
        }
        // medium_active_metall + water = oxyde + H2 (Heating is required)
        else if MEDIUM_ACTIVE_METALLS.contains(&me_name.as_str()) && heating {
            let metall_substance = get_substance_from_reagents(reagents, 1, 0).unwrap();
            let oxygen_substance = Substance::from_string("O2", p_t).unwrap();
            let oxyde =
                Self::reaction_me_antime(&vec![metall_substance, oxygen_substance], heating, p_t)
                    .unwrap()
                    .0
                    .iter()
                    .next()
                    .unwrap()
                    .clone();

            let hydrogen = Substance::from_string("H2", p_t).unwrap();

            return Ok((vec![oxyde, hydrogen], rtype));
        } else {
            // non_active_metall + water or no heating = no reaction
            return Ok((vec![], ReactionType::None));
        }
    }

    fn reaction_me_acid(
        reagents: &Vec<Substance>,
        heating: bool,
        p_t: &PeriodicTable,
    ) -> Result<(Vec<Substance>, ReactionType), &'static str> {
        let rtype = ReactionType::Substition;
        let (me_name, me_element) = get_simple_me_from_reagents(reagents);

        if ACTIVE_METALLS.contains(&me_name.as_str())
            || MEDIUM_ACTIVE_METALLS.contains(&me_name.as_str())
        {
            let (mut map, acid_r_oxydation) = get_acid_residue(reagents);

            let me_oxydation = *me_element.valencies.first().unwrap() as i8;
            let (me_index, acid_r_index) = calculate_indexes_for_2(me_oxydation, acid_r_oxydation);
            for (_, block) in map.iter_mut() {
                block.index *= acid_r_index;
            }

            map.insert(me_name, SB::new(me_element, me_index, 0));

            let substance_salt = Substance::from_elements(map).unwrap();
            let substance_hydrogen = Substance::from_string("H2", p_t).unwrap();

            return Ok((
                vec![substance_salt, substance_hydrogen],
                ReactionType::Substition,
            ));
        } else {
            return Ok((vec![], ReactionType::None));
        }
    }
}

fn get_simple_me_from_reagents(reagents: &Vec<Substance>) -> (String, Element) {
    let substance = get_substance_from_reagents(reagents, 1, 0).unwrap();
    let reagent_iter = substance.me.iter().next().unwrap();
    (reagent_iter.0.clone(), reagent_iter.1.element.clone())
}

fn get_simple_antime_from_reagents(reagents: &Vec<Substance>) -> (String, Element) {
    let substance = get_substance_from_reagents(reagents, 0, 1).unwrap();
    let reagent_iter = substance.anti_me.iter().next().unwrap();
    (reagent_iter.0.clone(), reagent_iter.1.element.clone())
}

fn get_substance_from_reagents(
    reagents: &Vec<Substance>,
    me_count: usize,
    ame_count: usize,
) -> Option<Substance> {
    for reagent in reagents {
        if reagent.me.len() == me_count && reagent.anti_me.len() == ame_count {
            return Some(reagent.clone());
        }
    }
    None
}

fn get_acid_residue(reagents: &Vec<Substance>) -> (HashMap<String, SB>, i8) {
    let mut map = HashMap::new();
    let mut acid_residue_valence = 0;
    for reagent in reagents {
        if reagent.anti_me.len() > 1 && reagent.me.len() == 0 {
            for (name, block) in reagent.anti_me.iter() {
                if block.element.charge != 1 {
                    // != H
                    map.insert(name.clone(), block.clone());
                } else {
                    acid_residue_valence = (block.index as i8) * -1;
                }
            }
        }
    }
    (map, acid_residue_valence)
}

// Calculation of indexes for two elements.
// Idea: element_index = LCM(first_valence, second_valence) / element_oxydation
fn calculate_indexes_for_2(first_oxydation: i8, second_oxydation: i8) -> (u8, u8) {
    let first_oxydation = first_oxydation.abs() as u8;
    let second_oxydation = second_oxydation.abs() as u8;
    let lcm = lcm(first_oxydation, second_oxydation);
    (lcm / first_oxydation, lcm / second_oxydation)
}
