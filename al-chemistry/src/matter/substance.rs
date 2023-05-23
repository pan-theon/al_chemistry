use super::element::Element;
use std::collections::BTreeSet;
use std::collections::HashMap;

use crate::math_util::gcd;
use crate::parser;
use crate::periodic_table::PeriodicTable;

// Electrochmical series of metalls. Where:
// from left to right the standard electrochemical potential increases
const METALLS: [&str; 29] = [
    "Li", "Cs", "Rb", "K", "Ba", "Sr", "Ca", "Na", "Mg", "Al", "Ti", "Mn", "Zn", "Cr", "Fe", "Cd",
    "Co", "Ni", "Sn", "Pb", "H", "Sb", "Bi", "Cu", "Hg", "Ag", "Pd", "Pt", "Au",
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SubstanceClass {
    Simple,
    SimpleMetall,
    Hydride,
    Oxide,
    Peroxide,
    Base,
    Acid,
    Salt,
}

// something atomic - atom, ion, group or remainder
#[derive(Debug, Clone)]
pub struct SubstanceBlock {
    // elements with indexes
    pub content: HashMap<String, (Element, u8)>,
    pub oxidation_state: i8,
}

#[derive(Debug, Clone)]
pub struct Substance {
    pub content: HashMap<String, (SubstanceBlock, u8)>,
    pub class: SubstanceClass,
}

impl PartialEq for Substance {
    fn eq(&self, other: &Self) -> bool {
        if self.class != other.class || self.content.len() != other.content.len() {
            return false;
        }
        for key in self.content.keys() {
            if !other.content.contains_key(key) {
                return false;
            }
        }
        true
    }
}

// The idea: Substance itself determines its class
// and calculates oxidation_states of its SubstanceBlocks
// (not the parser)
impl SubstanceBlock {
    pub fn new(content: HashMap<String, (Element, u8)>) -> Self {
        Self {
            content,
            // calculate after Substance was built
            oxidation_state: 0,
        }
    }
}

// Not one great distrubutor, but many small - one for every SubstanceClass
impl Substance {
    pub fn from_string(s: &str, p_t: &PeriodicTable) -> Result<Self, &'static str> {
        let mut e = parser::collect_elements(s, p_t)?;
        if e.len() != 1 {
            return Err(&"There's must be only one substance");
        }
        Self::from_elements(e.swap_remove(0))
    }
    pub fn from_elements(e: HashMap<String, (Element, u8)>) -> Result<Self, &'static str> {
        let checkers: Vec<
            fn(HashMap<String, (Element, u8)>) -> Result<Self, HashMap<String, (Element, u8)>>,
        > = vec![
            Self::try_hydride,
            Self::try_peroxide,
            Self::try_oxide,
            Self::try_base,
            Self::try_salt,
            Self::try_acid,
        ];
        let mut res = Self::try_simple(e);
        for checker in checkers {
            res = match res {
                Ok(s) => return Ok(s),
                Err(e) => checker(e),
            };
        }

        match res {
            Ok(s) => Ok(s),
            Err(_) => Err("Your substance is unknown"),
        }
    }

    fn try_simple(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        if e.len() != 1 {
            return Err(e);
        }

        let mut is_metall = false;
        let mut content = HashMap::<String, (SubstanceBlock, u8)>::new();
        let (k, mut v) = e.iter_mut().next().unwrap();
        let sb_index = v.1;
        v.1 = 1;

        if METALLS.contains(&k.as_str()) {
            is_metall = true;
        }

        content.insert(k.clone(), (SubstanceBlock::new(e), sb_index));

        if is_metall {
            return Ok(Self {
                content,
                class: SubstanceClass::SimpleMetall,
            });
        }

        Ok(Self {
            content,
            class: SubstanceClass::Simple,
        })
    }

    fn try_hydride(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        if e.len() != 2 {
            return Err(e);
        }

        let mut h = match e.remove_entry("H") {
            Some(el) => el,
            None => return Err(e),
        };

        let mut el = e.iter_mut().next().unwrap();
        if el.1 .0.electronegativity > h.1 .0.electronegativity {
            e.insert(h.0, h.1);
            return Err(e);
        }

        let indexes = [el.1 .1, h.1 .1];
        el.1 .1 = 1;
        h.1 .1 = 1;
        let h = HashMap::from([h]);

        let mut content = HashMap::<String, (SubstanceBlock, u8)>::new();
        content.insert(el.0.clone(), (SubstanceBlock::new(e), indexes[0]));
        content.insert("H".to_string(), (SubstanceBlock::new(h), indexes[1]));

        Ok(Self {
            content,
            class: SubstanceClass::Hydride,
        })
    }

    fn try_oxide(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        if e.len() != 2 {
            return Err(e);
        }

        let mut o = match e.remove_entry("O") {
            Some(el) => el,
            None => return Err(e),
        };

        let mut el = e.iter_mut().next().unwrap();
        let indexes = [el.1 .1, o.1 .1];
        el.1 .1 = 1;
        o.1 .1 = 1;
        let o = HashMap::from([o]);

        let mut content = HashMap::new();
        content.insert(el.0.clone(), (SubstanceBlock::new(e), indexes[0]));
        content.insert("O".to_string(), (SubstanceBlock::new(o), indexes[1]));

        Ok(Self {
            content,
            class: SubstanceClass::Oxide,
        })
    }

    fn try_peroxide(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        if e.len() != 2 {
            return Err(e);
        }

        let o2 = match e.remove_entry("O") {
            Some(el) => el,
            None => return Err(e),
        };

        let mut el = e.iter_mut().next().unwrap();
        // peroxides - it's for active Me only(exclude Be)
        if el.1 .0.group > 2
            || el.1 .0.group == el.1 .0.period
            || o2.1 .1 != 2
            || f64::from(el.1 .0.valencies[0]) * f64::from(el.1 .1) != 2.0
        {
            e.insert(o2.0, o2.1);
            return Err(e);
        }

        let el_index = el.1 .1;
        el.1 .1 = 1;
        let o2 = HashMap::from([o2]);

        let mut content = HashMap::new();
        content.insert(el.0.clone(), (SubstanceBlock::new(e), el_index));
        content.insert("02".to_string(), (SubstanceBlock::new(o2), 1));

        Ok(Self {
            content,
            class: SubstanceClass::Peroxide,
        })
    }

    fn try_base(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        // only hydroxides - inorganic bases
        if e.len() != 3 {
            return Err(e);
        }

        let mut o = match e.remove_entry("O") {
            Some(el) => el,
            None => return Err(e),
        };
        let mut h = match e.remove_entry("H") {
            Some(el) => el,
            None => {
                e.insert(o.0, o.1);
                return Err(e);
            }
        };

        let mut content = HashMap::new();
        let el = e.iter_mut().next().unwrap();
        // exception - NH₄OH
        if el.0 == "N" && el.1 .1 == 1 && o.1 .1 == 1 && h.1 .1 == 5 {
            let mut h4 = h.clone();
            h4.1 .1 = 4;
            e.insert(h4.0, h4.1);
            h.1 .1 = 1;
            content.insert("NH4".to_string(), (SubstanceBlock::new(e), 1));
            content.insert(
                "OH".to_string(),
                (SubstanceBlock::new(HashMap::from([o, h])), 1),
            );

            return Ok(Self {
                content,
                class: SubstanceClass::Base,
            });
        }

        if o.1.1 == h.1.1 &&
            // B-Si-As-Te-Po-Lv - border between Me and AntiMe
            ((el.1.0.period < 6 && el.1.0.group < 11 + el.1.0.period) && el.1.0.group < 16)
        {
            let el_index = el.1 .1;
            let oh_index = o.1 .1;
            o.1 .1 = 1;
            h.1 .1 = 1;
            let oh = HashMap::from([o, h]);
            content.insert(el.0.clone(), (SubstanceBlock::new(e), el_index));
            content.insert("OH".to_string(), (SubstanceBlock::new(oh), oh_index));

            return Ok(Self {
                content,
                class: SubstanceClass::Base,
            });
        }

        e.insert(o.0, o.1);
        e.insert(h.0, h.1);
        Err(e)
    }

    fn try_acid(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        let mut h = match e.remove_entry("H") {
            Some(el) => el,
            None => return Err(e),
        };

        let mut residue = String::new();
        // is there something with big electronegativity - e.g. O or S
        let mut is_oxidant = false;
        for el in e.iter() {
            if el.1 .0.group < 3 {
                e.insert(h.0, h.1);
                return Err(e);
            }
            if el.1 .0.group > 15 || el.1 .0.electronegativity > 2.8 {
                is_oxidant = true;
            }

            residue = format!("{}{}{}", residue, el.0, el.1 .1);
        }
        if !is_oxidant {
            e.insert(h.0, h.1);
            return Err(e);
        }

        let h_index = h.1 .1;
        h.1 .1 = 1;
        let mut content = HashMap::new();
        content.insert(
            "H".to_string(),
            (SubstanceBlock::new(HashMap::from([h])), h_index),
        );
        content.insert(residue, (SubstanceBlock::new(e), 1));

        Ok(Self {
            content,
            class: SubstanceClass::Acid,
        })
    }

    fn try_salt(
        mut e: HashMap<String, (Element, u8)>,
    ) -> Result<Self, HashMap<String, (Element, u8)>> {
        let mut Me = Vec::<(String, (Element, u8))>::new();
        let mut amphMe = Vec::<(String, (Element, u8))>::new();
        let mut antiMe = Vec::<(String, (Element, u8))>::new();

        let h = e.remove_entry("H");
        let o = e.remove_entry("O");
        for el in e.into_iter() {
            let kind;
            if el.1 .0.group < 3 {
                Me.push(el);
                continue;
            // antiMe sorted by electronegativity for oxidant determination
            // and so amphMe because of sth like CuTiO₃ or Fe₂(CrO₄)₃
            // acid-forming there is Me with less electronegativity
            } else if (el.1 .0.period < 6 && el.1 .0.group < 11 + el.1 .0.period)
                && el.1 .0.group < 16
            {
                kind = &mut amphMe;
            } else {
                kind = &mut antiMe;
            }

            let mut i = 0;
            while i < kind.len() && kind[i].1 .0.electronegativity < el.1 .0.electronegativity {
                i += 1;
            }
            kind.insert(i, el);
        }

        // there must be oxidant and Me
        let mut parts = [&mut antiMe, &mut Me];
        for part in &mut parts {
            if part.len() == 0 {
                if amphMe.len() == 0 {
                    return not_salt(Me, antiMe, amphMe, o, h);
                }
                part.push(amphMe.swap_remove(0));
            }
        }
        let mut content = HashMap::new();
        // and then rushed...
        if let Some(mut o) = o {
            let gcd = gcd(o.1 .1, antiMe[antiMe.len() - 1].1 .1);
            // try to decide whether it's base or acid salt
            if let Some(mut h) = h {
                // possible valencies of acid residue
                let mut h_vacant_e = Vec::<i16>::new();
                let mut oh_vacant_e = Vec::<i16>::new();
                let h_oh = [&mut h_vacant_e, &mut oh_vacant_e];
                for i in 0..2 {
                    let valencies = &antiMe[antiMe.len() - 1].1 .0.valencies;
                    let n_o = (i16::from(o.1 .1 / gcd) - i16::from(h.1 .1 * i)) << 1;
                    if n_o == 0 {
                        for v in valencies {
                            h_oh[i as usize].push(*v as i16);
                        }
                        continue;
                    }
                    for val in antiMe[antiMe.len() - 1].1 .0.valencies.iter() {
                        // abuse fact, that antiMe linked only with O
                        // full formula: val(antiMe) - 2 * n(=O)
                        // where n(=O) := val(antiMe) - n(O)
                        // and n(=O) - amount of doubly linked O - there's no link to them
                        let guess_vacant_e: i16 = n_o - *val as i16;
                        if guess_vacant_e >= 1
                            && guess_vacant_e <= *val as i16
                            && (guess_vacant_e & 1) == (*val & 1) as i16
                        {
                            h_oh[i as usize].push(guess_vacant_e);
                        }
                    }
                }
                if h_vacant_e.len() == 0 && oh_vacant_e.len() == 0 {
                    return not_salt(Me, antiMe, amphMe, Some(o), Some(h));
                }
                Me.append(&mut amphMe);
                let mut me = Vec::new();
                for i in 0..Me.len() {
                    if Me[i].1 .0.valencies.len() == 1 {
                        let me_valency = Me[i].1 .0.valencies[0] as i16;

                        h_vacant_e = h_vacant_e.iter().map(|x| x - me_valency).collect();
                        oh_vacant_e = oh_vacant_e.iter().map(|x| x - me_valency).collect();

                        me.push(Me.swap_remove(0));
                    }
                }
                h_vacant_e.retain(|&x| x != 0);
                oh_vacant_e.retain(|&x| x != 0);
                // no more vacancies but there's still H
                if Me.len() == 0 && h_vacant_e.len() == 0 && oh_vacant_e.len() == 0 {
                    Me.append(&mut me);
                    return not_salt(Me, antiMe, amphMe, Some(o), Some(h));
                }
                // Stars aligned, and so valencies of residue and other part of Salt
                if Me.len() == 0 && oh_vacant_e.contains(&(0 - h.1 .1 as i16)) {
                    let oh_index = h.1 .1;
                    o.1 .1 -= h.1 .1;
                    let mut o_oh = o.clone();
                    o_oh.1 .1 = 1;
                    h.1 .1 = 1;
                    content.insert(
                        "OH".to_string(),
                        (SubstanceBlock::new(HashMap::from([o_oh, h])), oh_index),
                    );
                } else if Me.len() == 0 && h_vacant_e.contains(&(h.1 .1 as i16)) {
                    let h_index = h.1 .1;
                    h.1 .1 = 1;
                    content.insert(
                        "H".to_string(),
                        (SubstanceBlock::new(HashMap::from([h])), h_index),
                    );
                } else {
                    // Sorting through Me's with variable valencies
                    // As Rust docs say: "Use the Set when ... You just want a set."
                    let mut variants = BTreeSet::new();
                    if Me.len() == 1 {
                        for v in Me[0].1 .0.valencies.iter() {
                            variants.insert((*v * Me[0].1 .1) as i16);
                        }
                    }
                    for i in 0..Me.len() {
                        for i_i in Me[i].1 .0.valencies.iter() {
                            for o in i + 1..Me.len() {
                                for o_i in Me[o].1 .0.valencies.iter() {
                                    variants.insert((i_i * Me[i].1 .1 + o_i * Me[o].1 .1) as i16);
                                }
                            }
                        }
                    }
                    let h_e: BTreeSet<i16> =
                        h_vacant_e.iter().map(|&x| x + h.1 .1 as i16).collect();
                    let h_intersection: Vec<&i16> = variants.intersection(&h_e).collect();

                    let oh_e: BTreeSet<i16> =
                        oh_vacant_e.iter().map(|&x| x - h.1 .1 as i16).collect();
                    let oh_intersection: Vec<&i16> = variants.intersection(&oh_e).collect();

                    // Well, it's base salt
                    if oh_intersection.len() != 0 {
                        let oh_index = h.1 .1;
                        o.1 .1 -= h.1 .1;
                        let mut o_oh = o.clone();
                        o_oh.1 .1 = 1;
                        h.1 .1 = 1;
                        content.insert(
                            "OH".to_string(),
                            (SubstanceBlock::new(HashMap::from([o_oh, h])), oh_index),
                        );
                    }
                    // And that's acid salt
                    else if h_intersection.len() != 0 {
                        let h_index = h.1 .1;
                        h.1 .1 = 1;
                        content.insert(
                            "H".to_string(),
                            (SubstanceBlock::new(HashMap::from([h])), h_index),
                        );
                    } else {
                        return not_salt(Me, antiMe, amphMe, Some(o), Some(h));
                    }
                }
                Me.append(&mut me);
            }
            antiMe.push(o);
        }
        // God does not play dice, but we do ;)
        if amphMe.len() > 0 {
            if antiMe.len() < 2 {
                antiMe.push(amphMe.swap_remove(amphMe.len() - 1));
            }
            Me.append(&mut amphMe);
        }
        // Whew! Now we need just to build the Salt
        for mut m in Me {
            let m_index = m.1 .1;
            m.1 .1 = 1;
            content.insert(
                m.0.clone(),
                (SubstanceBlock::new(HashMap::from([m])), m_index),
            );
        }
        let mut residue_name = String::from(&antiMe[0].0);
        let mut cd = antiMe[0].1 .1;
        for i in 1..antiMe.len() {
            cd = gcd(cd, antiMe[i].1 .1);
            residue_name += &antiMe[i].0;
        }
        let mut residue = HashMap::new();
        for mut am in antiMe {
            am.1 .1 /= cd;
            residue.insert(am.0, am.1);
        }
        content.insert(residue_name, (SubstanceBlock::new(residue), cd));

        Ok(Substance {
            content,
            class: SubstanceClass::Salt,
        })
    }
}

fn not_salt(
    me: Vec<(String, (Element, u8))>,
    amph_me: Vec<(String, (Element, u8))>,
    anti_me: Vec<(String, (Element, u8))>,
    h: Option<(String, (Element, u8))>,
    o: Option<(String, (Element, u8))>,
) -> Result<Substance, HashMap<String, (Element, u8)>> {
    let mut err = HashMap::new();

    for m in me {
        err.insert(m.0, m.1);
    }
    for m in anti_me {
        err.insert(m.0, m.1);
    }
    for m in amph_me {
        err.insert(m.0, m.1);
    }
    if let Some(o) = o {
        err.insert(o.0, o.1);
    }
    if let Some(h) = h {
        err.insert(h.0, h.1);
    }

    Err(err)
}
