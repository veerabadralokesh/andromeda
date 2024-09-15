use std::collections::HashMap;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let (mut ans, mut mask) = (0, 0);
        let mut prefixMap = HashMap::from([(0, -1)]);
        for (i, c) in s.chars().enumerate() {
            if "aeiou".contains(c) {
                mask ^= (1 << (c as u8 - b'a'));
            }
            match prefixMap.get(&mask) {
                None => {
                    prefixMap.insert(mask, i as i32);
                },
                Some(idx) => {
                    ans = ans.max(i as i32 - idx);
                }
            }
        }
        ans
    }
}

