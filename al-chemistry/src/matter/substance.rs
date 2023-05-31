use super::element::Element;
use std::collections::HashMap;
use std::fmt;

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
    Hydride,
    Oxide,
    Peroxide,
    Base,
    Acid,
    Salt,
}

#[derive(Debug, Clone)]
pub struct SubstanceBlock {
    // elements with indexes
    pub element: Element,
    pub index: u8,
    pub oxidation_state: i8,
}

// The idea: Substance itself determines its class
// and calculates oxidation_states of its SubstanceBlocks
// (not the parser)
impl SubstanceBlock {
    pub fn new(element: Element, index: u8, oxidation_state: i8) -> Self {
        Self {
            element,
            index,
            oxidation_state,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Substance {
    pub me: HashMap<String, SubstanceBlock>,
    pub anti_me: HashMap<String, SubstanceBlock>,
    pub class: SubstanceClass,
}

impl PartialEq for Substance {
    fn eq(&self, other: &Self) -> bool {
        if self.class != other.class
            || self.me.len() != other.me.len()
            || self.anti_me.len() != other.anti_me.len()
        {
            return false;
        }
        for m in self.me.keys() {
            if !other.me.contains_key(m) {
                return false;
            }
        }
        for a in self.anti_me.keys() {
            if !other.anti_me.contains_key(a) {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Substance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for group in [&self.me, &self.anti_me] {
            for (name, sb) in group {
                res = format!("{}{}{}", res, name, sb.index);
            }
        }
        write!(f, "{}", res)
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
    pub fn from_elements(sb: HashMap<String, SubstanceBlock>) -> Result<Self, &'static str> {
        let checkers: Vec<
            fn(HashMap<String, SubstanceBlock>) -> Result<Self, HashMap<String, SubstanceBlock>>,
        > = vec![
            Self::try_hydride,
            Self::try_peroxide,
            Self::try_oxide,
            Self::try_base,
            Self::try_salt,
            Self::try_acid,
        ];
        let mut res = Self::try_simple(sb);
        for checker in checkers {
            res = match res {
                Ok(s) => return Ok(s),
                Err(sb) => checker(sb),
            };
        }

        match res {
            Ok(s) => Ok(s),
            Err(_) => Err("Your substance is unknown"),
        }
    }

    fn try_simple(
        sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        if sbs.len() != 1 {
            return Err(sbs);
        }

        let sb = sbs.values().next().unwrap();

        match sb.element.is_me() {
            true => Ok(Self {
                me: sbs,
                anti_me: HashMap::new(),
                class: SubstanceClass::Simple,
            }),
            _ => Ok(Self {
                me: HashMap::new(),
                anti_me: sbs,
                class: SubstanceClass::Simple,
            }),
        }
    }

    fn try_hydride(
        mut sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        if sbs.len() != 2 {
            return Err(sbs);
        }

        let mut h = match sbs.remove_entry("H") {
            Some(sb) => sb,
            None => return Err(sbs),
        };

        let mut sb = sbs.drain().next().unwrap();
        if sb.1.element.electronegativity > h.1.element.electronegativity {
            sbs.insert(h.0, h.1);
            sbs.insert(sb.0, sb.1);
            return Err(sbs);
        }

        sb.1.oxidation_state = match other_oxy(-1 * h.1.index as i8, sb.1.index) {
            Some(oxy) => {
                if !sb.1.element.valencies.contains(&(oxy as u8)) {
                    sbs.insert(h.0, h.1);
                    sbs.insert(sb.0, sb.1);
                    return Err(sbs);
                }
                oxy
            },
            None => {
                sbs.insert(h.0, h.1);
                sbs.insert(sb.0, sb.1);
                return Err(sbs);
            }
        };
        h.1.oxidation_state = -1;

        let mut me = HashMap::new();
        let mut anti_me = HashMap::from([h]);

        match sb.1.element.is_me() {
            true => me.insert(sb.0, sb.1),
            _ => anti_me.insert(sb.0, sb.1),
        };

        Ok(Self {
            me,
            anti_me,
            class: SubstanceClass::Hydride,
        })
    }

    fn try_oxide(
        mut sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        if sbs.len() != 2 {
            return Err(sbs);
        }

        let mut o = match sbs.remove_entry("O") {
            Some(el) => el,
            None => return Err(sbs),
        };

        let mut sb = sbs.drain().next().unwrap();
        sb.1.oxidation_state = match other_oxy(-2 * o.1.index as i8, sb.1.index) {
            Some(oxy) => {
                if !sb.1.element.valencies.contains(&(oxy as u8)) {
                    sbs.insert(o.0, o.1);
                    sbs.insert(sb.0, sb.1);
                    return Err(sbs);
                }
                oxy
            },
            None => {
                sbs.insert(o.0, o.1);
                sbs.insert(sb.0, sb.1);
                return Err(sbs);
            }
        };
        o.1.oxidation_state = -2;

        let mut me = HashMap::new();
        let mut anti_me = HashMap::from([o]);
        match sb.1.element.is_me() {
            true => me.insert(sb.0, sb.1),
            false => anti_me.insert(sb.0, sb.1),
        };

        Ok(Self {
            me,
            anti_me,
            class: SubstanceClass::Oxide,
        })
    }

    fn try_peroxide(
        mut sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        if sbs.len() != 2 {
            return Err(sbs);
        }

        let mut o2 = match sbs.remove_entry("O") {
            Some(el) => el,
            None => return Err(sbs),
        };

        let mut sb = sbs.drain().next().unwrap();
        // peroxides - it's for active Me only(exclude Be)
        if sb.1.element.group > 2
            || sb.1.element.group == sb.1.element.period
            || o2.1.index != 2
            || f64::from(sb.1.element.valencies[0]) * f64::from(sb.1.index) != 2.0
        {
            sbs.insert(o2.0, o2.1);
            sbs.insert(sb.0, sb.1);
            return Err(sbs);
        }

        sb.1.oxidation_state = match other_oxy(-1 * o2.1.index as i8, sb.1.index) {
            Some(oxy) => {
                if !sb.1.element.valencies.contains(&(oxy as u8)) {
                    sbs.insert(o2.0, o2.1);
                    sbs.insert(sb.0, sb.1);
                    return Err(sbs);
                }
                oxy
            },
            None => {
                sbs.insert(o2.0, o2.1);
                sbs.insert(sb.0, sb.1);
                return Err(sbs);
            }
        };
        o2.1.oxidation_state = -1;

        let mut me = HashMap::new();
        let mut anti_me = HashMap::from([o2]);
        match sb.1.element.is_me() {
            true => me.insert(sb.0, sb.1),
            _ => anti_me.insert(sb.0, sb.1),
        };

        Ok(Self {
            me,
            anti_me,
            class: SubstanceClass::Peroxide,
        })
    }

    fn try_base(
        mut sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        // only hydroxides - inorganic bases
        if sbs.len() != 3 {
            return Err(sbs);
        }

        let mut o = match sbs.remove_entry("O") {
            Some(el) => el,
            None => return Err(sbs),
        };
        let mut h = match sbs.remove_entry("H") {
            Some(el) => el,
            None => {
                sbs.insert(o.0, o.1);
                return Err(sbs);
            }
        };

        let mut sb = sbs.drain().next().unwrap();
        h.1.oxidation_state = 1;
        o.1.oxidation_state = -2;

        // exception - NH₄OH
        if sb.0 == "N" && sb.1.index == 1 && o.1.index == 1 && h.1.index == 5 {
            sb.1.oxidation_state = -3;

            return Ok(Self {
                me: HashMap::new(),
                anti_me: HashMap::from([sb, o, h]),
                class: SubstanceClass::Base,
            });
        }

        if o.1.index == h.1.index && sb.1.element.is_me() {
            sb.1.oxidation_state = match other_oxy(-1 * o.1.index as i8, sb.1.index) {
                Some(oxy) => {
                    if !sb.1.element.valencies.contains(&(oxy as u8)) {
                        sbs.insert(o.0, o.1);
                        sbs.insert(h.0, h.1);
                        sbs.insert(sb.0, sb.1);
                        return Err(sbs);
                    }
                    oxy
                },
                None => {
                    sbs.insert(o.0, o.1);
                    sbs.insert(h.0, h.1);
                    sbs.insert(sb.0, sb.1);
                    return Err(sbs);
                }
            };

            return Ok(Self {
                me: HashMap::from([sb]),
                anti_me: HashMap::from([o, h]),
                class: SubstanceClass::Base,
            });
        }

        sbs.insert(o.0, o.1);
        sbs.insert(h.0, h.1);
        sbs.insert(sb.0, sb.1);
        Err(sbs)
    }

    fn try_acid(
        mut sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        let mut h = match sbs.remove_entry("H") {
            Some(el) => el,
            None => return Err(sbs),
        };
        h.1.oxidation_state = 1;

        // is there something with big electronegativity - e.g. O or S
        let mut ox_eln = 0_f32;
        let mut oxidant = String::new();
        for sb in &mut sbs {
            if sb.1.element.group < 3 {
                return wrong_class(vec![], vec![Some(h)]);
            }
            if (sb.1.element.group > 15 || sb.1.element.electronegativity > 2.8)
                && ox_eln < sb.1.element.electronegativity
            {
                ox_eln = sb.1.element.electronegativity;
                oxidant = sb.0.clone();
            }
        }
        let mut ox = match oxidant.is_empty() {
            false => sbs.remove_entry(&oxidant).unwrap(),
            _ => return wrong_class(vec![], vec![Some(h)]),
        };
        ox.1.oxidation_state = ox.1.element.group as i8 - 18;

        let (res_vals, len) =
            valency_variants(&sbs, ox.1.oxidation_state as i16 * ox.1.index as i16);

        match res_vals.iter().position(|&x| -x == h.1.index as i16) {
            Some(i) => valencies_by_variant(&mut sbs, i, len),
            _ => return wrong_class(vec![], vec![Some(h), Some(ox)]),
        }

        let mut me = HashMap::new();
        let mut anti_me = HashMap::from([h, ox]);
        for sb in sbs {
            match sb.1.element.is_me() {
                true => me.insert(sb.0, sb.1),
                false => anti_me.insert(sb.0, sb.1),
            };
        }

        Ok(Self {
            me,
            anti_me,
            class: SubstanceClass::Acid,
        })
    }

    fn try_salt(
        mut sbs: HashMap<String, SubstanceBlock>,
    ) -> Result<Self, HashMap<String, SubstanceBlock>> {
        let mut me = HashMap::new();
        let mut anti_me = HashMap::new();

        let h = sbs.remove_entry("H");
        let o = sbs.remove_entry("O");

        // oxidant and salt-forming Me for case of O
        // in case of sth like CuTiO₃ or Fe₂(CrO₄)₃
        // salt-forming there is Me with less electronegativity
        let mut importants = [String::new(), String::new()];
        let mut importants_eln = [0_f32, 10_f32];
        for sb in sbs {
            let sb_eln = sb.1.element.electronegativity;
            if sb.1.element.is_me() {
                if sb.1.element.group > 2 && sb_eln < importants_eln[1] {
                    importants[1] = sb.0.clone();
                    importants_eln[1] = sb_eln;
                }
                me.insert(sb.0, sb.1);
                continue;
            }
            if sb.1.element.group > 15 || sb_eln > 2.8 && importants_eln[0] < sb_eln {
                importants[0] = sb.0.clone();
                importants_eln[0] = sb_eln;
            }
            anti_me.insert(sb.0, sb.1);
        }

        // there must be oxidant and Me
        if me.len() == 0 {
            return wrong_class(vec![anti_me], vec![h, o]);
        }
        let mut ox = match o {
            Some(o) => o,
            None => {
                match importants[0].is_empty() {
                    true => return wrong_class(vec![me, anti_me], vec![h]),
                    _ => anti_me.remove_entry(&importants[0]).unwrap(),
                }
            },
        };
        ox.1.oxidation_state = (ox.1.element.group as i8 - 18) * ox.1.index as i8;
        if anti_me.len() == 0 {
            if !importants[1].is_empty() {
                let m = me.remove_entry(&importants[1]).unwrap();
                anti_me.insert(m.0, m.1);
            }
        }

        let (mut mes_valency_variants, mut me_len);
        let (mut res_valency_variants, mut res_len);
        let mut h_save = None;
        let mut me_start = 0;
        if let Some(mut h) = h {
            println!("kuku");
            h.1.oxidation_state = 1;
            // try base salt
            if ox.1.element.charge == 8
                && (ox.1.index > h.1.index || !importants[0].is_empty())
            {
                ox.1.oxidation_state = -2;
                let mut o = None;
                let mut res_start;
                if ox.1.index > h.1.index {
                    res_start = (h.1.index as i16 - ox.1.index as i16) << 1;
                }
                else {
                    let mut oxi = anti_me.remove_entry(&importants[0]).unwrap();
                    oxi.1.oxidation_state = oxi.1.element.group as i8 - 18;

                    res_start = oxi.1.index as i16 * oxi.1.oxidation_state as i16;
                    o = Some(oxi);
                }
                (mes_valency_variants, me_len) = valency_variants(&me, -(h.1.index as i16));
                (res_valency_variants, res_len) = valency_variants(&anti_me, res_start);
                for i in 0..mes_valency_variants.len() {
                    for j in 0..res_valency_variants.len() {
                        if mes_valency_variants[i] == -res_valency_variants[j] {
                            valencies_by_variant(&mut me, i, me_len);
                            valencies_by_variant(&mut anti_me, j, res_len);
                            if let Some(o) = o {
                                anti_me.insert(o.0, o.1);
                            }
                            anti_me.insert(ox.0, ox.1);
                            anti_me.insert(h.0, h.1);
                            return Ok(Self {
                                me,
                                anti_me,
                                class: SubstanceClass::Salt,
                            });
                        }
                    }
                }
                if let Some(o) = o {
                    anti_me.insert(o.0, o.1);
                }
                println!("{:#?}", anti_me);
            }
            me_start = h.1.index as i16;
            h_save = Some(h);
        }
        (mes_valency_variants, me_len) = valency_variants(&me, me_start);
        (res_valency_variants, res_len) = valency_variants(&anti_me, ox.1.oxidation_state as i16);
        for i in 0..mes_valency_variants.len() {
            for j in 0..res_valency_variants.len() {
                if mes_valency_variants[i] == -res_valency_variants[j] {
                    valencies_by_variant(&mut me, i, me_len);
                    valencies_by_variant(&mut anti_me, j, res_len);
                    anti_me.insert(ox.0, ox.1);
                    if let Some(h) = h_save {
                        anti_me.insert(h.0, h.1);
                    }
                    return Ok(Self {
                        me,
                        anti_me,
                        class: SubstanceClass::Salt,
                    });
                }
            }
        }

        wrong_class(vec![me, anti_me], vec![h_save, Some(ox)])
    }
}

fn wrong_class(
    groups: Vec<HashMap<String, SubstanceBlock>>,
    alones: Vec<Option<(String, SubstanceBlock)>>,
) -> Result<Substance, HashMap<String, SubstanceBlock>> {
    let mut err = HashMap::new();
    for group in groups {
        for sb in group {
            err.insert(sb.0, sb.1);
        }
    }
    for alone in alones {
        if let Some(sb) = alone {
            err.insert(sb.0, sb.1);
        }
    }

    Err(err)
}

fn other_oxy(c_oxy: i8, o_idx: u8) -> Option<i8> {
    let c_oxy = c_oxy as i16;
    let o_idx = o_idx as i16;
    match c_oxy % o_idx {
        0 => Some(-(c_oxy / o_idx) as i8),
        _ => None,
    }
}

fn valency_variants(sbs: &HashMap<String, SubstanceBlock>, start: i16) -> (Vec<i16>, usize) {
    let mut len = 1;
    for sb in sbs.values() {
        len *= sb.element.valencies.len();
    }
    let l = len;
    let mut variants = vec![start; len];
    for sb in sbs.values() {
        let val_n = sb.element.valencies.len();
        for i in 0..l / len {
            for v_i in 0..val_n {
                for j in 0..len / val_n {
                    let idx = i * l / len + v_i * len / val_n + j;
                    variants[idx] += (sb.element.valencies[v_i] * sb.index) as i16;
                }
            }
        }
        len /= val_n;
    }

    (variants, l)
}

fn valencies_by_variant(sbs: &mut HashMap<String, SubstanceBlock>, variant: usize, mut len: usize) {
    for mut sb in sbs.values_mut() {
        let mut val_idx = variant % len;
        len /= sb.element.valencies.len();
        val_idx /= len;

        sb.oxidation_state = sb.element.valencies[val_idx] as i8;
    }
}
