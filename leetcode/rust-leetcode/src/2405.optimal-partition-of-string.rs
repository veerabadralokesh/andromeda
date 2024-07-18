use std::collections::HashSet;
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut counts = HashSet::new();
        let mut ans = 1i32;
        for b in s.bytes() {
            if counts.contains(&b) {
                ans += 1;
                counts.clear();
            }
            counts.insert(b);
        }
        ans
    }
}

/*

*/

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut res = 1;
        let mut mask = 0;
        for c in s.chars() {
            let cur = 1 << (c as u8 - b'a');
            if (mask & cur) == cur {
                res += 1;
                mask = 0;
            }
            mask ^= cur;
        }
        res
    }
}
