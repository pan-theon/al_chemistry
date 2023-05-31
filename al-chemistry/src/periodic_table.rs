use std::collections::HashMap;

use crate::matter::element::Element;

pub struct PeriodicTable {
    table: HashMap<String, Element>,
}

impl PeriodicTable {
    pub fn new() -> PeriodicTable {
        let table = HashMap::from([
            ("H".to_string(), Element {charge:1, group:1, period:1, a_rm:1.0079, valencies: vec![1], electronegativity:2.2}),
            ("He".to_string(), Element {charge:2, group:18, period:1, a_rm:4.0026, valencies: vec![8], electronegativity:0.0}),

            ("Li".to_string(), Element {charge:3, group:1, period:2, a_rm:6.941, valencies: vec![1], electronegativity:0.98}),
            ("Be".to_string(), Element {charge:4, group:2, period:2, a_rm:9.01218, valencies: vec![1,2], electronegativity:1.57}),
            ("B".to_string(), Element {charge:5, group:13, period:2, a_rm:10.81, valencies: vec![3], electronegativity:2.04}),
            ("C".to_string(), Element {charge:6, group:14, period:2, a_rm:12.011, valencies: vec![2,4], electronegativity:2.55}),
            ("N".to_string(), Element {charge:7, group:15, period:2, a_rm:14.0067, valencies: vec![1,2,3,4], electronegativity:3.04}),
            ("O".to_string(), Element {charge:8, group:16, period:2, a_rm:15.9994, valencies: vec![2], electronegativity:3.44}),
            ("F".to_string(), Element {charge:9, group:17, period:2, a_rm:18.9984, valencies: vec![1], electronegativity:3.98}),
            ("Ne".to_string(), Element {charge:10, group:18, period:2, a_rm:20.1797, valencies: vec![0], electronegativity:0.0}),

            ("Na".to_string(), Element {charge:11, group:1, period:3, a_rm:22.98977, valencies: vec![1], electronegativity:0.93}),
            ("Mg".to_string(), Element {charge:12, group:2, period:3, a_rm:24.305, valencies: vec![2], electronegativity:1.31}),
            ("Al".to_string(), Element {charge:13, group:13, period:3, a_rm:26.9815, valencies: vec![3], electronegativity:1.61}),
            ("Si".to_string(), Element {charge:14, group:14, period:3, a_rm:28.0855, valencies: vec![2,4], electronegativity:1.9}),
            ("P".to_string(), Element {charge:15, group:15, period:3, a_rm:30.97376, valencies: vec![3,5], electronegativity:2.19}),
            ("S".to_string(), Element {charge:16, group:16, period:3, a_rm:32.065, valencies: vec![2,4,6], electronegativity:2.58}),
            ("Cl".to_string(), Element {charge:17, group:17, period:3, a_rm:35.453, valencies: vec![1,3,5,7], electronegativity:3.16}),
            ("Ar".to_string(), Element {charge:18, group:18, period:3, a_rm:39.948, valencies: vec![0], electronegativity:0.0}),

            ("K".to_string(), Element {charge:19, group:1, period:4, a_rm:39.0983, valencies: vec![1], electronegativity:0.82}),
            ("Ca".to_string(), Element {charge:20, group:2, period:4, a_rm:40.078, valencies: vec![2], electronegativity:1.0}),
            ("Sc".to_string(), Element {charge:21, group:3, period:4, a_rm:44.9559, valencies: vec![3], electronegativity:1.36}),
            ("Ti".to_string(), Element {charge:22, group:4, period:4, a_rm:47.867, valencies: vec![2,3,4], electronegativity:1.54}),
            ("V".to_string(), Element {charge:23, group:5, period:4, a_rm:50.9415, valencies: vec![2,3,4,5], electronegativity:1.63}),
            ("Cr".to_string(), Element {charge:24, group:6, period:4, a_rm:51.996, valencies: vec![2,3,6], electronegativity:1.66}),
            ("Mn".to_string(), Element {charge:25, group:7, period:4, a_rm:54.938, valencies: vec![2,4,6,7], electronegativity:1.55}),
            ("Fe".to_string(), Element {charge:26, group:8, period:4, a_rm:55.845, valencies: vec![2,3], electronegativity:1.83}),
            ("Co".to_string(), Element {charge:27, group:9, period:4, a_rm:58.9332, valencies: vec![2,3], electronegativity:1.88}),
            ("Ni".to_string(), Element {charge:28, group:10, period:4, a_rm:58.6934, valencies: vec![2,3], electronegativity:1.91}),
            ("Cu".to_string(), Element {charge:29, group:11, period:4, a_rm:63.546, valencies: vec![1,2], electronegativity:1.9}),
            ("Zn".to_string(), Element {charge:30, group:12, period:4, a_rm:65.409, valencies: vec![2], electronegativity:1.65}),
            ("Ga".to_string(), Element {charge:31, group:13, period:4, a_rm:69.723, valencies: vec![1,2,3], electronegativity:1.81}),
            ("Ge".to_string(), Element {charge:32, group:14, period:4, a_rm:72.64, valencies: vec![2,4], electronegativity:2.01}),
            ("As".to_string(), Element {charge:33, group:15, period:4, a_rm:74.9216, valencies: vec![3,5], electronegativity:2.18}),
            ("Se".to_string(), Element {charge:34, group:16, period:4, a_rm:78.96, valencies: vec![2,4,6], electronegativity:2.55}),
            ("Br".to_string(), Element {charge:35, group:17, period:4, a_rm:79.904, valencies: vec![1,3,5,7], electronegativity:2.96}),
            ("Kr".to_string(), Element {charge:36, group:18, period:4, a_rm:83.798, valencies: vec![0], electronegativity:3.0}),

            ("Rb".to_string(), Element {charge:37, group:1, period:5, a_rm:85.4678, valencies: vec![1], electronegativity:0.82}),
            ("Sr".to_string(), Element {charge:38, group:2, period:5, a_rm:87.62, valencies: vec![2], electronegativity:0.95}),
            ("Y".to_string(), Element {charge:39, group:3, period:5, a_rm:88.906, valencies: vec![3], electronegativity:1.22}),
            ("Zr".to_string(), Element {charge:40, group:4, period:5, a_rm:91.224, valencies: vec![2,3,4], electronegativity:1.33}),
            ("Nb".to_string(), Element {charge:41, group:5, period:5, a_rm:92.9064, valencies: vec![1,2,3,4,5], electronegativity:1.6}),
            ("Mo".to_string(), Element {charge:42, group:6, period:5, a_rm:95.94, valencies: vec![2,3,4,5,6], electronegativity:2.16}),
            ("Tc".to_string(), Element {charge:43, group:7, period:5, a_rm:98.0, valencies: vec![2,3,4,5,6,7], electronegativity:1.9}),
            ("Ru".to_string(), Element {charge:44, group:8, period:5, a_rm:101.07, valencies: vec![2,3,4,5,6,7,8], electronegativity:2.2}),
            ("Rh".to_string(), Element {charge:45, group:9, period:5, a_rm:102.9055, valencies: vec![2,3,4,5,6], electronegativity:2.28}),
            ("Pd".to_string(), Element {charge:46, group:10, period:5, a_rm:106.42, valencies: vec![2,4], electronegativity:2.2}),
            ("Ag".to_string(), Element {charge:47, group:11, period:5, a_rm:107.8682, valencies: vec![1,2,3], electronegativity:1.93}),
            ("Cd".to_string(), Element {charge:48, group:12, period:5, a_rm:112.41, valencies: vec![1,2], electronegativity:1.69}),
            ("In".to_string(), Element {charge:49, group:13, period:5, a_rm:114.818, valencies: vec![1,2,3], electronegativity:1.78}),
            ("Sn".to_string(), Element {charge:50, group:14, period:5, a_rm:118.71, valencies: vec![2,4], electronegativity:1.96}),
            ("Sb".to_string(), Element {charge:51, group:15, period:5, a_rm:121.76, valencies: vec![3,5], electronegativity:2.05}),
            ("Te".to_string(), Element {charge:52, group:16, period:5, a_rm:127.6, valencies: vec![2,4,6], electronegativity:2.1}),
            ("I".to_string(), Element {charge:53, group:17, period:5, a_rm:126.9045, valencies: vec![1,3,5,7], electronegativity:2.66}),
            ("Xe".to_string(), Element {charge:54, group:18, period:5, a_rm:131.29, valencies: vec![0], electronegativity:2.6}),

            ("Cs".to_string(), Element {charge:55, group:1, period:6, a_rm:132.9054, valencies: vec![1], electronegativity:0.79}),
            ("Ba".to_string(), Element {charge:56, group:2, period:6, a_rm:137.327, valencies: vec![2], electronegativity:0.89}),
            ("La".to_string(), Element {charge:57, group:3, period:6, a_rm:138.9055, valencies: vec![3], electronegativity:1.1}),
            ("Ce".to_string(), Element {charge:58, group:3, period:6, a_rm:140.116, valencies: vec![3,4], electronegativity:1.12}),
            ("Pr".to_string(), Element {charge:59, group:3, period:6, a_rm:140.9076, valencies: vec![2,3,4], electronegativity:1.13}),
            ("Nd".to_string(), Element {charge:60, group:3, period:6, a_rm:144.242, valencies: vec![2,3], electronegativity:1.14}),
            ("Pm".to_string(), Element {charge:61, group:3, period:6, a_rm:145.0, valencies: vec![3], electronegativity:1.13}),
            ("Sm".to_string(), Element {charge:62, group:3, period:6, a_rm:150.36, valencies: vec![2,3], electronegativity:1.17}),
            ("Eu".to_string(), Element {charge:63, group:3, period:6, a_rm:151.964, valencies: vec![2,3], electronegativity:1.2}),
            ("Gd".to_string(), Element {charge:64, group:3, period:6, a_rm:157.25, valencies: vec![2,3], electronegativity:1.2}),
            ("Tb".to_string(), Element {charge:65, group:3, period:6, a_rm:158.9253, valencies: vec![2,3,4], electronegativity:1.2}),
            ("Dy".to_string(), Element {charge:66, group:3, period:6, a_rm:162.5, valencies: vec![2,3], electronegativity:1.22}),
            ("Ho".to_string(), Element {charge:67, group:3, period:6, a_rm:165.9303, valencies: vec![3], electronegativity:1.23}),
            ("Er".to_string(), Element {charge:68, group:3, period:6, a_rm:167.259, valencies: vec![3], electronegativity:1.24}),
            ("Tm".to_string(), Element {charge:69, group:3, period:6, a_rm:168.9342, valencies: vec![2,3], electronegativity:1.25}),
            ("Yb".to_string(), Element {charge:70, group:3, period:6, a_rm:173.04, valencies: vec![2,3], electronegativity:1.1}),
            ("Lu".to_string(), Element {charge:71, group:3, period:6, a_rm:174.967, valencies: vec![3], electronegativity:1.27}),
            ("Hf".to_string(), Element {charge:72, group:4, period:6, a_rm:178.49, valencies: vec![1,2,3,4], electronegativity:1.3}),
            ("Ta".to_string(), Element {charge:73, group:5, period:6, a_rm:180.9479, valencies: vec![1,2,3,4,5], electronegativity:1.5}),
            ("W".to_string(), Element {charge:74, group:6, period:6, a_rm:183.84, valencies: vec![2,3,4,5,6], electronegativity:2.36}),
            ("Re".to_string(), Element {charge:75, group:7, period:6, a_rm:186.207, valencies: vec![1,2,3,4,5,6,7], electronegativity:1.9}),
            ("Os".to_string(), Element {charge:76, group:8, period:6, a_rm:190.23, valencies: vec![1,2,3,4,5,6,7,8], electronegativity:2.2}),
            ("Ir".to_string(), Element {charge:77, group:9, period:6, a_rm:192.217, valencies: vec![1,2,3,4,5,6], electronegativity:2.2}),
            ("Pt".to_string(), Element {charge:78, group:10, period:6, a_rm:195.085, valencies: vec![2,3,4,5,6], electronegativity:2.28}),
            ("Au".to_string(), Element {charge:79, group:11, period:6, a_rm:196.96657, valencies: vec![1,2,3,5], electronegativity:2.54}),
            ("Hg".to_string(), Element {charge:80, group:12, period:6, a_rm:200.59, valencies: vec![1,2], electronegativity:2.0}),
            ("Tl".to_string(), Element {charge:81, group:13, period:6, a_rm:204.3833, valencies: vec![1,2,3], electronegativity:1.62}),
            ("Pb".to_string(), Element {charge:82, group:14, period:6, a_rm:207.2, valencies: vec![2,4], electronegativity:2.33}),
            ("Bi".to_string(), Element {charge:83, group:15, period:6, a_rm:208.9804, valencies: vec![3,5], electronegativity:2.02}),
            ("Po".to_string(), Element {charge:84, group:16, period:6, a_rm:209.0, valencies: vec![2,4,6], electronegativity:2.0}),

            ("Rn".to_string(), Element {charge:86, group:18, period:6, a_rm:222.0, valencies: vec![0], electronegativity:0.0}),

            ("Fr".to_string(), Element {charge:87, group:1, period:7, a_rm:223.0, valencies: vec![1], electronegativity:0.7}),
            ("Ra".to_string(), Element {charge:88, group:2, period:7, a_rm:226.0, valencies: vec![2], electronegativity:0.89}),
            ("Ac".to_string(), Element {charge:89, group:3, period:7, a_rm:227.0, valencies: vec![3], electronegativity:1.1}),
            ("Th".to_string(), Element {charge:90, group:3, period:7, a_rm:232.038, valencies: vec![2,3,4], electronegativity:1.3}),
            ("Pa".to_string(), Element {charge:91, group:3, period:7, a_rm:231.0359, valencies: vec![2,3,4,5], electronegativity:1.5}),
            ("U".to_string(), Element {charge:92, group:3, period:7, a_rm:238.0289, valencies: vec![3,4,5,6], electronegativity:1.38}),
            ("Np".to_string(), Element {charge:93, group:3, period:7, a_rm:237.0, valencies: vec![3,4,5,6,7], electronegativity:1.36}),
            ("Pu".to_string(), Element {charge:94, group:3, period:7, a_rm:244.0, valencies: vec![3,4,5,6,7], electronegativity:1.28}),
            ("Am".to_string(), Element {charge:95, group:3, period:7, a_rm:243.0, valencies: vec![2,3,4,5,6], electronegativity:1.3}),
            ("Cm".to_string(), Element {charge:96, group:3, period:7, a_rm:247.0, valencies: vec![2,3,4], electronegativity:1.3}),
            ("Bk".to_string(), Element {charge:97, group:3, period:7, a_rm:247.0, valencies: vec![3,4], electronegativity:1.3}),
            ("Cf".to_string(), Element {charge:98, group:3, period:7, a_rm:251.0, valencies: vec![2,3,4], electronegativity:1.3}),
            ("Es".to_string(), Element {charge:99, group:3, period:7, a_rm:252.0, valencies: vec![2,3], electronegativity:1.3}),
            ("Fm".to_string(), Element {charge:100, group:3, period:7, a_rm:257.0, valencies: vec![2,3], electronegativity:1.3}),
        ]);
        Self { table }
    }

    pub fn get(&self, key: &str) -> Option<&Element> {
        self.table.get(key)
    }

    pub fn insert(&mut self, name: String, el: Element)
    {
        for (k, v) in &self.table {
            if *k == name || v.charge == el.charge {
                return;
            }
        }
        self.table.insert(name, el);
    }

    pub fn remove(&mut self, name: &str) -> Option<(String, Element)> {
        self.table.remove_entry(name)
    }
}
