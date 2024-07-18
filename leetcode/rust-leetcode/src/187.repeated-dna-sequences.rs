use std::collections::*;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut map = HashMap::with_capacity(s.len());
        for i in 0..s.len()-9 {
            *map.entry(&s[i..i+10]).or_insert(0) += 1;
        }
        // println!("{:?}", map);
        map.into_iter().filter(|&(k, v)| v > 1).map(|(k,v)| k.to_string()).collect()
    }
}

