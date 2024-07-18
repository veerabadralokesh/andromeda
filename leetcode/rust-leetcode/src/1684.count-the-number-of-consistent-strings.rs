const a:usize = 'a' as usize;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut dict = [false; 26];
        for b in allowed.bytes() {
            dict[b as usize - a] = true
        }

        words
            .iter()
            .filter(|word| word.bytes().all(|b| dict[b as usize - a]))
            .count() as i32
    }
}

use std::collections::HashSet;
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut ans = 0i32;
        let mut hs = HashSet::new();
        for b in allowed.bytes() {hs.insert(b);};
        let mut flag:bool = true;
        for word in words {
            flag = true;
            for b in word.bytes() {
                if !hs.contains(&b) {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
        ans
    }
}