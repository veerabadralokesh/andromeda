use std::collections::HashMap;
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let (mut left, mut right, mut mid, mut count) = (0, s.len()-1, 0, 0);
        let mut ans = String::new();
        let mut map = HashMap::new();
        while left <= right {
            count = 0;
            mid = left + (right-left)/2;
            for i in 0..=s.len()-mid {
                let mut c = map.entry(&s[i..i+mid]).or_insert(0);
                *c += 1;
                if *c > 1 {
                    count = *c;
                    ans = String::from(&s[i..i+mid]);
                    break;
                }
            }
            map.clear();
            if count > 1 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}

