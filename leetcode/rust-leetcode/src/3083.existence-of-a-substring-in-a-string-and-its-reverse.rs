use std::collections::*;
impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut set = HashSet::with_capacity(s.len());
        for i in 0..s.len()-1 {
            set.insert(&s[i..i+2]);
        }
        let s = s.chars().rev().collect::<String>();
        for i in 0..s.len()-1 {
            if set.contains(&s[i..i+2]) {
                return true;
            }
        }
        false
    }
}

