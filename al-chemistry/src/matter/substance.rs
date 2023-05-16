use super::element::Element;
use std::collections::HashMap;

use crate::parser;
use crate::periodic_table::PeriodicTable;

#[derive(PartialEq)]
pub enum SubstanceClass {
    Simple,
    Hydride,
    Oxide,
    Peroxide,
    Base,
    Acid,
    Salt,
}

// something atomic - atom, ion, group or remainder
pub struct SubstanceBlock {
    // elements with indexes
    pub content: HashMap<String, (Element, u8)>,
    pub oxidation_state: i8,
}

pub struct Substance {
    pub content: HashMap<String, (SubstanceBlock, u8)>,
    pub class: SubstanceClass,
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
            Self::try_oxide,
            Self::try_peroxide,
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

        let mut content = HashMap::<String, (SubstanceBlock, u8)>::new();
        let (k, mut v) = e.iter_mut().next().unwrap();
        let sb_index = v.1;
        v.1 = 1;

        content.insert(k.clone(), (SubstanceBlock::new(e), sb_index));
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
        if antiMe.len() == 0
            || (Me.len() == 0 && amphMe.len() == 0)
            || (antiMe[antiMe.len() - 1].1 .0.group <= 15
                && antiMe[antiMe.len() - 1].1 .0.electronegativity < 2.8
                && o.is_none())
        {
            Me.append(&mut antiMe);
            Me.append(&mut amphMe);
            if let Some(h) = h {
                Me.push(h);
            }
            if let Some(o) = o {
                Me.push(o);
            }

            let mut err = HashMap::new();
            for m in Me {
                err.insert(m.0, m.1);
            }
            return Err(err);
        }
        if Me.len() == 0 {
            Me.push(amphMe.swap_remove(0));
        }
        let mut content = HashMap::new();
        // and then rushed...
        if let Some(mut o) = o {
            if antiMe.len() == 0 {
                if amphMe.len() == 0 {
                    Me.push(o);
                    if let Some(h) = h {
                        Me.push(h);
                    }

                    let mut err = HashMap::new();
                    for m in Me {
                        err.insert(m.0, m.1);
                    }
                    return Err(err);
                }
                antiMe.push(amphMe.swap_remove(0))
            }
            let gcd = gcd(o.1 .1, antiMe[antiMe.len() - 1].1 .1);
            // try to decide whether it's base or acid salt
            if let Some(mut h) = h {
                // possible valencies of acid residue
                let mut vacant_es = [0_i16, 0_i16];
                for i in 0..2 {
                    let n_o = (i16::from(o.1 .1 / gcd) - i16::from(h.1 .1 * i)) << 1;
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
                            vacant_es[usize::from(i)] = guess_vacant_e as i16;
                            break;
                        }
                    }
                }
                if vacant_es[0] == 0 && vacant_es[1] == 0 {
                    Me.append(&mut antiMe);
                    Me.append(&mut vec![o, h]);

                    let mut err = HashMap::new();
                    for m in Me {
                        err.insert(m.0, m.1);
                    }
                    return Err(err);
                }
                Me.append(&mut amphMe);
                let mut me = Vec::new();
                for i in 0..Me.len() {
                    if Me[i].1 .0.valencies.len() == 1 {
                        vacant_es[0] -= Me[i].1 .0.valencies[0] as i16;
                        vacant_es[1] -= Me[i].1 .0.valencies[0] as i16;

                        me.push(Me.swap_remove(0));
                    }
                }
                // no more vacancies but there's still H
                if Me.len() == 0 && vacant_es[0] == 0 && vacant_es[1] == 0 {
                    antiMe.append(&mut me);
                    antiMe.append(&mut vec![o, h]);

                    let mut err = HashMap::new();
                    for e in antiMe {
                        err.insert(e.0, e.1);
                    }
                    return Err(err);
                }
                // Stars aligned, and so valencies of residue and other part of Salt
                if Me.len() == 0 && -vacant_es[1] == h.1 .1 as i16 {
                    let oh_index = h.1 .1;
                    o.1 .1 -= h.1 .1;
                    let mut o_oh = o.clone();
                    o_oh.1 .1 = 1;
                    h.1 .1 = 1;
                    content.insert(
                        "OH".to_string(),
                        (SubstanceBlock::new(HashMap::from([o_oh, h])), oh_index),
                    );
                } else if Me.len() == 0 && vacant_es[0] == h.1 .1 as i16 {
                    let h_index = h.1 .1;
                    h.1 .1 = 1;
                    content.insert(
                        "H".to_string(),
                        (SubstanceBlock::new(HashMap::from([h])), h_index),
                    );
                } else {
                    // Sorting through Me's with variable valencies
                    // As Rust docs say: "Use the Set when ... You just want a set."
                    let mut variants = std::collections::BTreeSet::new();
                    if Me.len() == 1 {
                        for v in Me[0].1 .0.valencies.iter() {
                            variants.insert((Me[0].1 .1 * *v) as i16);
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
                    // Well, it's base salt
                    if variants.contains(&(vacant_es[1] + h.1 .1 as i16)) {
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
                    else if variants.contains(&(vacant_es[0] - h.1 .1 as i16)) {
                        let h_index = h.1 .1;
                        h.1 .1 = 1;
                        content.insert(
                            "H".to_string(),
                            (SubstanceBlock::new(HashMap::from([h])), h_index),
                        );
                    } else {
                        Me.append(&mut me);
                        Me.append(&mut antiMe);
                        Me.append(&mut vec![o, h]);

                        let mut err = HashMap::new();
                        for m in Me {
                            err.insert(m.0, m.1);
                        }
                        return Err(err);
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

fn gcd(mut a: u8, mut b: u8) -> u8 {
    if b > a {
        (a, b) = (b, a);
    }
    match (a, b) {
        (0, x) | (x, 0) => x,
        (1, _) | (_, 1) => 1,
        (a, b) => gcd(b, a % b),
    }
}
