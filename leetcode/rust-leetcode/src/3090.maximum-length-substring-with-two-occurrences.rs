use std::collections::*;
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let sc = s.chars().collect::<Vec<char>>();
        let mut counts = HashMap::new();
        let (mut ans, mut j) = (0, 0);
        for i in 0..sc.len() {
            *counts.entry(sc[i]).or_insert(0) += 1;
            while 2 < *counts.get(&sc[i]).unwrap() {
                *counts.entry(sc[j]).or_insert(0) -= 1;
                j += 1;
            }
            ans = ans.max(i-j+1);
        }
        ans as _
    }
}

