// pure chemical element
#[derive(Clone, Debug)]
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
        let mut res = false;
        if self.period < 6 {
            if self.group < 11 + self.period {
                res = true;
            }
        } else if self.group < 16 {
            res = true;
        }
        res
    }
}
