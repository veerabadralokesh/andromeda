use std::collections::HashMap;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut hm:HashMap<char, i32> = HashMap::with_capacity(26);
        for c in sentence.chars() {
            hm.insert(c, 1);
        }
        hm.len() == 26
    }
}