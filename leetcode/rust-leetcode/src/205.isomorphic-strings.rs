use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map:HashMap<u8, u8> = HashMap::new();
        let mut mapr:HashMap<u8, u8> = HashMap::new();
        let sba = s.into_bytes();
        let tba = t.into_bytes();
        for i in 0..tba.len() {
            let tb = tba[i];
            let sb = sba[i];
            if map.contains_key(&tb) {
                if *map.get(&tb).unwrap() != sb {
                    return false;
                }
            } else {
                map.insert(tb, sb);
            }
            if mapr.contains_key(&sb) {
                if *mapr.get(&sb).unwrap() != tb {
                    return false;
                }
            } else {
                mapr.insert(sb, tb);
            }
        }
        true
    }
}
