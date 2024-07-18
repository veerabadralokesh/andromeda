use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pb = pattern.into_bytes();
        let sw = s.split_whitespace().collect::<Vec<&str>>();
        if pb.len() != sw.len() {
            return false;
        }
        let mut map:HashMap<u8, &str> = HashMap::new();
        let mut mapr:HashMap<&str, u8> = HashMap::new();
        for (i, b) in pb.iter().enumerate() {
            if map.contains_key(b) {
                if *map.get(b).unwrap() != sw[i] {
                    return false;
                }
            } else {
                map.insert(*b, sw[i]);
            }
            if mapr.contains_key(&sw[i]) {
                if *mapr.get(&sw[i]).unwrap() != *b {
                    return false;
                }
            } else {
                mapr.insert(sw[i], *b);
            }
        }
        true
    }
}