use serde::{Deserialize, Serialize};

// pure chemical element
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Element {
    pub charge: u16,
    pub group: u8,
    pub period: u8,
    pub a_rm: f64,
    pub valencies: Vec<u8>,
    pub electronegativity: f32,
}

impl Element {
    pub fn is_me(&self) -> bool {
        (self.period < 6 && self.group < 11 + self.period) && self.group < 16
    }
}
