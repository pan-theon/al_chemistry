use super::element::Element;
use std::collections::HashMap;
use std::fmt;

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
            /*
            Self::try_salt,
            Self::try_acid,
            */
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
            Some(oxy) => oxy,
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
            Some(oxy) => oxy,
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
            Some(oxy) => oxy,
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
                Some(oxy) => oxy,
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

    /*
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
            (SubstanceBlock::new(HashMap::from([h]), -1), h_index),
        );
        content.insert(residue, (SubstanceBlock::new(e, -(h_index as i8)), 1));

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
        let parts = [&mut antiMe, &mut Me];
        for i in 0..2 {
            if parts[i].len() == 0 {
                if amphMe.len() == 0 {
                    return not_salt(Me, antiMe, amphMe, o, h);
                }
                let idx = match i {
                    1 => amphMe.len() - 1,
                    n => n,
                };
                parts[i].push(amphMe.swap_remove(idx));
            }
        }
        let mut content = HashMap::new();
        let mut me_idx = -1_i16;
        let mes_val = mes_val_variants(&Me, &amphMe);
        if let Some(mut h) = h.clone() {
            if let Some(mut o) = o {
                let h_o = o.clone();
                let mut oh_o = o.clone();

                antiMe.push(h_o);
                let h_residue_oxis = residue_oxis_variants(&antiMe, h.1 .1 as i16);
                antiMe.pop();

                let o_g_h = o.1 .1 > h.1 .1;
                if o_g_h {
                    oh_o.1 .1 -= h.1 .1;
                    antiMe.push(oh_o);
                }
                let oh_residue_oxis = residue_oxis_variants(&antiMe, -(h.1 .1 as i16));
                if o_g_h {
                    antiMe.pop();
                }

                let mut salt_type = "";
                for oh_e in oh_residue_oxis {
                    for m_i in 0..mes_val.len() {
                        if -oh_e == mes_val[m_i] {
                            me_idx = m_i as i16;
                            salt_type = "OH";
                            break;
                        }
                    }
                }
                if salt_type.is_empty() {
                    for h_e in h_residue_oxis {
                        for m_i in 0..mes_val.len() {
                            if -h_e == mes_val[m_i] {
                                me_idx = m_i as i16;
                                salt_type = "H";
                                break;
                            }
                        }
                    }
                }
                match salt_type {
                    "OH" => {
                        o.1 .1 -= h.1 .1;
                        let mut o_oh = o.clone();
                        let oh_idx = h.1 .1;
                        o_oh.1 .1 = 1;
                        h.1 .1 = 1;

                        content.insert(
                            "OH".to_string(),
                            (SubstanceBlock::new(HashMap::from([o_oh, h]), -1), oh_idx),
                        );
                    }
                    "H" => {
                        let h_idx = h.1 .1;
                        h.1 .1 = 1;
                        content.insert(
                            "H".to_string(),
                            (SubstanceBlock::new(HashMap::from([h]), 1), h_idx),
                        );
                    }
                    _ => return not_salt(Me, amphMe, antiMe, Some(o), Some(h)),
                };
                if o.1 .1 > 0 {
                    antiMe.push(o);
                }
                return Ok(build_salt(Me, amphMe, me_idx as usize, antiMe, content));
            }
            let h_idx = h.1 .1;
            h.1 .1 = 1;
            content.insert(
                "H".to_string(),
                (SubstanceBlock::new(HashMap::from([h]), 1), h_idx),
            );
        }
        if let Some(o) = o.clone() {
            antiMe.push(o);
        }
        let residue_oxis = residue_oxis_variants(&antiMe, 0);
        for res_i in residue_oxis {
            for me_i in 0..mes_val.len() {
                if -res_i == mes_val[me_i] {
                    me_idx = me_i as i16;
                }
            }
        }
        match me_idx {
            -1 => not_salt(Me, amphMe, antiMe, o, h),
            _ => Ok(build_salt(Me, amphMe, me_idx as usize, antiMe, content)),
        }
    }
    */
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

fn other_oxy(c_oxy: i8, o_idx: u8) -> Option<i8> {
    let c_oxy = c_oxy as i16;
    let o_idx = o_idx as i16;
    match c_oxy % o_idx {
        0 => Some(-(c_oxy / o_idx) as i8),
        _ => None,
    }
}

/*
fn mes_val_variants(
    me: &Vec<(String, (Element, u8))>,
    amphMe: &Vec<(String, (Element, u8))>,
) -> Vec<i16> {
    let mut me_val = 0_i16;
    for (_, m) in me {
        me_val += (m.0.valencies[0] * m.1) as i16;
    }

    if amphMe.len() == 0 {
        return vec![me_val];
    }
    let mut len = 1;
    for m in amphMe {
        len *= m.1 .0.valencies.len();
    }
    let l = len;
    let mut mes_val = vec![me_val; len];
    for m in amphMe {
        for i in 0..l / len {
            for v_i in 0..m.1 .0.valencies.len() {
                for j in 0..len / m.1 .0.valencies.len() {
                    let idx = i * l / len + v_i * len / m.1 .0.valencies.len() + j;
                    mes_val[idx] += (m.1 .0.valencies[0] * m.1 .1) as i16;
                }
            }
        }
        len /= m.1 .0.valencies.len();
    }

    mes_val
}

fn residue_oxis_variants(antiMe: &Vec<(String, (Element, u8))>, start: i16) -> Vec<i16> {
    let oxidant = &antiMe[antiMe.len() - 1].1;
    let oxidant_oxy = (oxidant.0.group as i16 - 18) * oxidant.1 as i16;

    let mut len = 1;
    for i in 0..antiMe.len() - 1 {
        len *= antiMe[i].1 .0.valencies.len();
    }
    let l = len;
    let mut residue_oxis = vec![oxidant_oxy + start; len];
    for a in 0..antiMe.len() - 1 {
        for i in 0..l / len {
            for v_i in 0..antiMe[a].1 .0.valencies.len() {
                for j in 0..len / antiMe[a].1 .0.valencies.len() {
                    let idx = i * l / len + v_i * len / antiMe[a].1 .0.valencies.len() + j;
                    residue_oxis[idx] += (antiMe[a].1 .0.valencies[v_i] * antiMe[a].1 .1) as i16;
                }
            }
        }
        len /= antiMe[a].1 .0.valencies.len();
    }

    residue_oxis
}

fn build_salt(
    Me: Vec<(String, (Element, u8))>,
    amphMe: Vec<(String, (Element, u8))>,
    amphs_val_variant: usize,
    antiMe: Vec<(String, (Element, u8))>,
    mut content: HashMap<String, (SubstanceBlock, u8)>,
) -> Substance {
    let mut oxy = 0;
    for mut m in Me {
        let m_oxy = m.1 .0.valencies[0] as i8;
        oxy += m_oxy;

        let m_idx = m.1 .1;
        m.1 .1 = 1;
        content.insert(
            m.0.clone(),
            (SubstanceBlock::new(HashMap::from([m]), m_oxy), m_idx),
        );
    }

    let mut len = 1;
    for (_, m) in &amphMe {
        len *= m.0.valencies.len();
    }
    for mut m in amphMe {
        let mut m_val_idx = amphs_val_variant % len;
        len /= m.1 .0.valencies.len();
        m_val_idx /= len;
        let m_oxy = m.1 .0.valencies[m_val_idx] as i8;
        oxy += m_oxy;

        let m_idx = m.1 .1;
        m.1 .1 = 1;
        content.insert(
            m.0.clone(),
            (SubstanceBlock::new(HashMap::from([m]), m_oxy), m_idx),
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
    content.insert(residue_name, (SubstanceBlock::new(residue, -oxy), cd));

    Substance {
        content,
        class: SubstanceClass::Salt,
    }
}
*/
