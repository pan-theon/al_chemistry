use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::matter::element::Element;

extern crate serde;
extern crate serde_json;

#[derive(Deserialize)]
pub struct PeriodicTable {
    table: HashMap<String, Element>,
}

impl PeriodicTable {
    pub fn new() -> Result<PeriodicTable, Box<dyn std::error::Error>> {
        let table = serde_json::from_str(DEFAULT_PERIOCID_TABLE)?;
        Ok(PeriodicTable { table })
    }

    pub fn from(path: &str) -> Result<PeriodicTable, Box<dyn std::error::Error>> {
        let table = fs::read_to_string(path)?;
        let table = serde_json::from_str(&table)?;
        Ok(PeriodicTable { table })
    }

    pub fn get(&self, key: &str) -> Option<&Element> {
        self.table.get(key)
    }
}

const DEFAULT_PERIOCID_TABLE: &str = "
{
    \"H\": {\"name\":\"H\", \"charge\":1, \"group\":1, \"period\":1, \"a_rm\":1.0079, \"valencies\":[1], \"electronegativity\":2.2},
    \"He\": {\"name\":\"He\", \"charge\":2, \"group\":18, \"period\":1, \"a_rm\":4.0026, \"valencies\":[8], \"electronegativity\":0.0},
    
    \"Li\": {\"name\":\"Li\", \"charge\":3, \"group\":1, \"period\":2, \"a_rm\":6.941, \"valencies\":[1], \"electronegativity\":0.98},
    \"Be\": {\"name\":\"Be\", \"charge\":4, \"group\":2, \"period\":2, \"a_rm\":9.01218, \"valencies\":[1,2], \"electronegativity\":1.57},
    \"B\": {\"name\":\"B\", \"charge\":5, \"group\":13, \"period\":2, \"a_rm\":10.81, \"valencies\":[3], \"electronegativity\":2.04},
    \"C\": {\"name\":\"C\", \"charge\":6, \"group\":14, \"period\":2, \"a_rm\":12.011, \"valencies\":[2,4], \"electronegativity\":2.55},
    \"N\": {\"name\":\"N\", \"charge\":7, \"group\":15, \"period\":2, \"a_rm\":14.0067, \"valencies\":[1,2,3,4], \"electronegativity\":3.04},
    \"O\": {\"name\":\"O\", \"charge\":8, \"group\":16, \"period\":2, \"a_rm\":15.9994, \"valencies\":[2], \"electronegativity\":3.44},
    \"F\": {\"name\":\"F\", \"charge\":9, \"group\":17, \"period\":2, \"a_rm\":18.9984, \"valencies\":[1], \"electronegativity\":3.98},
    \"Ne\": {\"name\":\"Ne\", \"charge\":10, \"group\":18, \"period\":2, \"a_rm\":20.1797, \"valencies\":[0], \"electronegativity\":0.0},
    
    \"Na\": {\"name\":\"Na\", \"charge\":11, \"group\":1, \"period\":3, \"a_rm\":22.98977, \"valencies\":[1], \"electronegativity\":0.93},
    \"Mg\": {\"name\":\"Mg\", \"charge\":12, \"group\":2, \"period\":3, \"a_rm\":24.305, \"valencies\":[2], \"electronegativity\":1.31},
    \"Al\": {\"name\":\"Al\", \"charge\":13, \"group\":13, \"period\":3, \"a_rm\":26.9815, \"valencies\":[3], \"electronegativity\":1.61},
    \"Si\": {\"name\":\"Si\", \"charge\":14, \"group\":14, \"period\":3, \"a_rm\":28.0855, \"valencies\":[2,4], \"electronegativity\":1.9},
    \"P\": {\"name\":\"P\", \"charge\":15, \"group\":15, \"period\":3, \"a_rm\":30.97376, \"valencies\":[3,5], \"electronegativity\":2.19},
    \"S\": {\"name\":\"S\", \"charge\":16, \"group\":16, \"period\":3, \"a_rm\":32.065, \"valencies\":[2,4,6], \"electronegativity\":2.58},
    \"Cl\": {\"name\":\"Cl\", \"charge\":17, \"group\":17, \"period\":3, \"a_rm\":35.453, \"valencies\":[1,3,5,7], \"electronegativity\":3.16},
    \"Ar\": {\"name\":\"Ar\", \"charge\":18, \"group\":18, \"period\":3, \"a_rm\":39.948, \"valencies\":[0], \"electronegativity\":0.0},
    
    \"K\": {\"name\":\"K\", \"charge\":19, \"group\":1, \"period\":4, \"a_rm\":39.0983, \"valencies\":[1], \"electronegativity\":0.82},
    \"Ca\": {\"name\":\"Ca\", \"charge\":20, \"group\":2, \"period\":4, \"a_rm\":40.078, \"valencies\":[2], \"electronegativity\":1.0},
    \"Sc\": {\"name\":\"Sc\", \"charge\":21, \"group\":3, \"period\":4, \"a_rm\":44.9559, \"valencies\":[3], \"electronegativity\":1.36},
    \"Ti\": {\"name\":\"Ti\", \"charge\":22, \"group\":4, \"period\":4, \"a_rm\":47.867, \"valencies\":[2,3,4], \"electronegativity\":1.54},
    \"V\": {\"name\":\"V\", \"charge\":23, \"group\":5, \"period\":4, \"a_rm\":50.9415, \"valencies\":[2,3,4,5], \"electronegativity\":1.63},
    \"Cr\": {\"name\":\"Cr\", \"charge\":24, \"group\":6, \"period\":4, \"a_rm\":51.996, \"valencies\":[2,3,6], \"electronegativity\":1.66},
    \"Mn\": {\"name\":\"Mn\", \"charge\":25, \"group\":7, \"period\":4, \"a_rm\":54.938, \"valencies\":[2,4,6,7], \"electronegativity\":1.55},
    \"Fe\": {\"name\":\"Fe\", \"charge\":26, \"group\":8, \"period\":4, \"a_rm\":55.845, \"valencies\":[2,3], \"electronegativity\":1.83},
    \"Co\": {\"name\":\"Co\", \"charge\":27, \"group\":9, \"period\":4, \"a_rm\":58.9332, \"valencies\":[2,3], \"electronegativity\":1.88},
    \"Ni\": {\"name\":\"Ni\", \"charge\":28, \"group\":10, \"period\":4, \"a_rm\":58.6934, \"valencies\":[2,3], \"electronegativity\":1.91},
    \"Cu\": {\"name\":\"Cu\", \"charge\":29, \"group\":11, \"period\":4, \"a_rm\":63.546, \"valencies\":[1,2], \"electronegativity\":1.9},
    \"Zn\": {\"name\":\"Zn\", \"charge\":30, \"group\":12, \"period\":4, \"a_rm\":65.409, \"valencies\":[2], \"electronegativity\":1.65},
    \"Ga\": {\"name\":\"Ga\", \"charge\":31, \"group\":13, \"period\":4, \"a_rm\":69.723, \"valencies\":[1,2,3], \"electronegativity\":1.81},
    \"Ge\": {\"name\":\"Ge\", \"charge\":32, \"group\":14, \"period\":4, \"a_rm\":72.64, \"valencies\":[2,4], \"electronegativity\":2.01},
    \"As\": {\"name\":\"As\", \"charge\":33, \"group\":15, \"period\":4, \"a_rm\":74.9216, \"valencies\":[3,5], \"electronegativity\":2.18},
    \"Se\": {\"name\":\"Se\", \"charge\":34, \"group\":16, \"period\":4, \"a_rm\":78.96, \"valencies\":[2,4,6], \"electronegativity\":2.55},
    \"Br\": {\"name\":\"Br\", \"charge\":35, \"group\":17, \"period\":4, \"a_rm\":79.904, \"valencies\":[1,3,5,7], \"electronegativity\":2.96},
    \"Kr\": {\"name\":\"Kr\", \"charge\":36, \"group\":18, \"period\":4, \"a_rm\":83.798, \"valencies\":[0], \"electronegativity\":3.0},
    
    \"Rb\": {\"name\":\"Rb\", \"charge\":37, \"group\":1, \"period\":5, \"a_rm\":85.4678, \"valencies\":[1], \"electronegativity\":0.82},
    \"Sr\": {\"name\":\"Sr\", \"charge\":38, \"group\":2, \"period\":5, \"a_rm\":87.62, \"valencies\":[2], \"electronegativity\":0.95},
    \"Y\": {\"name\":\"Y\", \"charge\":39, \"group\":3, \"period\":5, \"a_rm\":88.906, \"valencies\":[3], \"electronegativity\":1.22},
    \"Zr\": {\"name\":\"Zr\", \"charge\":40, \"group\":4, \"period\":5, \"a_rm\":91.224, \"valencies\":[2,3,4], \"electronegativity\":1.33},
    \"Nb\": {\"name\":\"Nb\", \"charge\":41, \"group\":5, \"period\":5, \"a_rm\":92.9064, \"valencies\":[1,2,3,4,5], \"electronegativity\":1.6},
    \"Mo\": {\"name\":\"Mo\", \"charge\":42, \"group\":6, \"period\":5, \"a_rm\":95.94, \"valencies\":[2,3,4,5,6], \"electronegativity\":2.16},
    \"Tc\": {\"name\":\"Tc\", \"charge\":43, \"group\":7, \"period\":5, \"a_rm\":98, \"valencies\":[2,3,4,5,6,7], \"electronegativity\":1.9},
    \"Ru\": {\"name\":\"Ru\", \"charge\":44, \"group\":8, \"period\":5, \"a_rm\":101.07, \"valencies\":[2,3,4,5,6,7,8], \"electronegativity\":2.2},
    \"Rh\": {\"name\":\"Rh\", \"charge\":45, \"group\":9, \"period\":5, \"a_rm\":102.9055, \"valencies\":[2,3,4,5,6], \"electronegativity\":2.28},
    \"Pd\": {\"name\":\"Pd\", \"charge\":46, \"group\":10, \"period\":5, \"a_rm\":106.42, \"valencies\":[2,4], \"electronegativity\":2.2},
    \"Ag\": {\"name\":\"Ag\", \"charge\":47, \"group\":11, \"period\":5, \"a_rm\":107.8682, \"valencies\":[1,2,3], \"electronegativity\":1.93},
    \"Cd\": {\"name\":\"Cd\", \"charge\":48, \"group\":12, \"period\":5, \"a_rm\":112.41, \"valencies\":[1,2], \"electronegativity\":1.69},
    \"In\": {\"name\":\"In\", \"charge\":49, \"group\":13, \"period\":5, \"a_rm\":114.818, \"valencies\":[1,2,3], \"electronegativity\":1.78},
    \"Sn\": {\"name\":\"Sn\", \"charge\":50, \"group\":14, \"period\":5, \"a_rm\":118.71, \"valencies\":[2,4], \"electronegativity\":1.96},
    \"Sb\": {\"name\":\"Sb\", \"charge\":51, \"group\":15, \"period\":5, \"a_rm\":121.76, \"valencies\":[3,5], \"electronegativity\":2.05},
    \"Te\": {\"name\":\"Te\", \"charge\":52, \"group\":16, \"period\":5, \"a_rm\":127.6, \"valencies\":[2,4,6], \"electronegativity\":2.1},
    \"I\": {\"name\":\"I\", \"charge\":53, \"group\":17, \"period\":5, \"a_rm\":126.9045, \"valencies\":[1,3,5,7], \"electronegativity\":2.66},
    \"Xe\": {\"name\":\"Xe\", \"charge\":54, \"group\":18, \"period\":5, \"a_rm\":131.29, \"valencies\":[0], \"electronegativity\":2.6},
    
    \"Cs\": {\"name\":\"Cs\", \"charge\":55, \"group\":1, \"period\":6, \"a_rm\":132.9054, \"valencies\":[1], \"electronegativity\":0.79},
    \"Ba\": {\"name\":\"Ba\", \"charge\":56, \"group\":2, \"period\":6, \"a_rm\":137.327, \"valencies\":[2], \"electronegativity\":0.89},
    \"La\": {\"name\":\"La\", \"charge\":57, \"group\":3, \"period\":6, \"a_rm\":138.9055, \"valencies\":[3], \"electronegativity\":1.1},
    \"Ce\": {\"name\":\"Ce\", \"charge\":58, \"group\":3, \"period\":6, \"a_rm\":140.116, \"valencies\":[3,4], \"electronegativity\":1.12},
    \"Pr\": {\"name\":\"Pr\", \"charge\":59, \"group\":3, \"period\":6, \"a_rm\":140.9076, \"valencies\":[2,3,4], \"electronegativity\":1.13},
    \"Nd\": {\"name\":\"Nd\", \"charge\":60, \"group\":3, \"period\":6, \"a_rm\":144.242, \"valencies\":[2,3], \"electronegativity\":1.14},
    \"Pm\": {\"name\":\"Pm\", \"charge\":61, \"group\":3, \"period\":6, \"a_rm\":145, \"valencies\":[3], \"electronegativity\":1.13},
    \"Sm\": {\"name\":\"Sm\", \"charge\":62, \"group\":3, \"period\":6, \"a_rm\":150.36, \"valencies\":[2,3], \"electronegativity\":1.17},
    \"Eu\": {\"name\":\"Eu\", \"charge\":63, \"group\":3, \"period\":6, \"a_rm\":151.964, \"valencies\":[2,3], \"electronegativity\":1.2},
    \"Gd\": {\"name\":\"Gd\", \"charge\":64, \"group\":3, \"period\":6, \"a_rm\":157.25, \"valencies\":[2,3], \"electronegativity\":1.2},
    \"Tb\": {\"name\":\"Tb\", \"charge\":65, \"group\":3, \"period\":6, \"a_rm\":158.9253, \"valencies\":[2,3,4], \"electronegativity\":1.2},
    \"Dy\": {\"name\":\"Dy\", \"charge\":66, \"group\":3, \"period\":6, \"a_rm\":162.5, \"valencies\":[2,3], \"electronegativity\":1.22},
    \"Ho\": {\"name\":\"Ho\", \"charge\":67, \"group\":3, \"period\":6, \"a_rm\":165.9303, \"valencies\":[3], \"electronegativity\":1.23},
    \"Er\": {\"name\":\"Er\", \"charge\":68, \"group\":3, \"period\":6, \"a_rm\":167.259, \"valencies\":[3], \"electronegativity\":1.24},
    \"Tm\": {\"name\":\"Tm\", \"charge\":69, \"group\":3, \"period\":6, \"a_rm\":168.9342, \"valencies\":[2,3], \"electronegativity\":1.25},
    \"Yb\": {\"name\":\"Yb\", \"charge\":70, \"group\":3, \"period\":6, \"a_rm\":173.04, \"valencies\":[2,3], \"electronegativity\":1.1},
    \"Lu\": {\"name\":\"Lu\", \"charge\":71, \"group\":3, \"period\":6, \"a_rm\":174.967, \"valencies\":[3], \"electronegativity\":1.27},
    \"Hf\": {\"name\":\"Hf\", \"charge\":72, \"group\":4, \"period\":6, \"a_rm\":178.49, \"valencies\":[1,2,3,4], \"electronegativity\":1.3},
    \"Ta\": {\"name\":\"Ta\", \"charge\":73, \"group\":5, \"period\":6, \"a_rm\":180.9479, \"valencies\":[1,2,3,4,5], \"electronegativity\":1.5},
    \"W\": {\"name\":\"W\", \"charge\":74, \"group\":6, \"period\":6, \"a_rm\":183.84, \"valencies\":[2,3,4,5,6], \"electronegativity\":2.36},
    \"Re\": {\"name\":\"Re\", \"charge\":75, \"group\":7, \"period\":6, \"a_rm\":186.207, \"valencies\":[1,2,3,4,5,6,7], \"electronegativity\":1.9},
    \"Os\": {\"name\":\"Os\", \"charge\":76, \"group\":8, \"period\":6, \"a_rm\":190.23, \"valencies\":[1,2,3,4,5,6,7,8], \"electronegativity\":2.2},
    \"Ir\": {\"name\":\"Ir\", \"charge\":77, \"group\":9, \"period\":6, \"a_rm\":192.217, \"valencies\":[1,2,3,4,5,6], \"electronegativity\":2.2},
    \"Pt\": {\"name\":\"Pt\", \"charge\":78, \"group\":10, \"period\":6, \"a_rm\":195.085, \"valencies\":[2,3,4,5,6], \"electronegativity\":2.28},
    \"Au\": {\"name\":\"Au\", \"charge\":79, \"group\":11, \"period\":6, \"a_rm\":196.96657, \"valencies\":[1,2,3,5], \"electronegativity\":2.54},
    \"Hg\": {\"name\":\"Hg\", \"charge\":80, \"group\":12, \"period\":6, \"a_rm\":200.59, \"valencies\":[1,2], \"electronegativity\":2.0},
    \"Tl\": {\"name\":\"Tl\", \"charge\":81, \"group\":13, \"period\":6, \"a_rm\":204.3833, \"valencies\":[1,2,3], \"electronegativity\":1.62},
    \"Pb\": {\"name\":\"Pb\", \"charge\":82, \"group\":14, \"period\":6, \"a_rm\":207.2, \"valencies\":[2,4], \"electronegativity\":2.33},
    \"Bi\": {\"name\":\"Bi\", \"charge\":83, \"group\":15, \"period\":6, \"a_rm\":208.9804, \"valencies\":[3,5], \"electronegativity\":2.02},
    \"Po\": {\"name\":\"Po\", \"charge\":84, \"group\":16, \"period\":6, \"a_rm\":209, \"valencies\":[2,4,6], \"electronegativity\":2.0},
    
    \"Rn\": {\"name\":\"Rn\", \"charge\":86, \"group\":18, \"period\":6, \"a_rm\":222, \"valencies\":[0], \"electronegativity\":0.0},
    
    \"Fr\": {\"name\":\"Fr\", \"charge\":87, \"group\":1, \"period\":7, \"a_rm\":223, \"valencies\":[1], \"electronegativity\":0.7},
    \"Ra\": {\"name\":\"Ra\", \"charge\":88, \"group\":2, \"period\":7, \"a_rm\":226, \"valencies\":[2], \"electronegativity\":0.89},
    \"Ac\": {\"name\":\"Ac\", \"charge\":89, \"group\":3, \"period\":7, \"a_rm\":227, \"valencies\":[3], \"electronegativity\":1.1},
    \"Th\": {\"name\":\"Th\", \"charge\":90, \"group\":3, \"period\":7, \"a_rm\":232.038, \"valencies\":[2,3,4], \"electronegativity\":1.3},
    \"Pa\": {\"name\":\"Pa\", \"charge\":91, \"group\":3, \"period\":7, \"a_rm\":231.0359, \"valencies\":[2,3,4,5], \"electronegativity\":1.5},
    \"U\": {\"name\":\"U\", \"charge\":92, \"group\":3, \"period\":7, \"a_rm\":238.0289, \"valencies\":[3,4,5,6], \"electronegativity\":1.38},
    \"Np\": {\"name\":\"Np\", \"charge\":93, \"group\":3, \"period\":7, \"a_rm\":237, \"valencies\":[3,4,5,6,7], \"electronegativity\":1.36},
    \"Pu\": {\"name\":\"Pu\", \"charge\":94, \"group\":3, \"period\":7, \"a_rm\":244, \"valencies\":[3,4,5,6,7], \"electronegativity\":1.28},
    \"Am\": {\"name\":\"Am\", \"charge\":95, \"group\":3, \"period\":7, \"a_rm\":243, \"valencies\":[2,3,4,5,6], \"electronegativity\":1.3},
    \"Cm\": {\"name\":\"Cm\", \"charge\":96, \"group\":3, \"period\":7, \"a_rm\":247, \"valencies\":[2,3,4], \"electronegativity\":1.3},
    \"Bk\": {\"name\":\"Bk\", \"charge\":97, \"group\":3, \"period\":7, \"a_rm\":247, \"valencies\":[3,4], \"electronegativity\":1.3},
    \"Cf\": {\"name\":\"Cf\", \"charge\":98, \"group\":3, \"period\":7, \"a_rm\":251, \"valencies\":[2,3,4], \"electronegativity\":1.3},
    \"Es\": {\"name\":\"Es\", \"charge\":99, \"group\":3, \"period\":7, \"a_rm\":252, \"valencies\":[2,3], \"electronegativity\":1.3},
    \"Fm\": {\"name\":\"Fm\", \"charge\":100, \"group\":3, \"period\":7, \"a_rm\":257, \"valencies\":[2,3], \"electronegativity\":1.3}
    }    
";
