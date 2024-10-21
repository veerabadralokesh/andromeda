use std::collections::HashSet;
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut set = HashSet::with_capacity(16);
        fn backtrack(i: usize, set: &mut HashSet<String>, ans: &mut usize, s: &String) {
            if i == s.len() {
                *ans = (*ans).max(set.len());
                return;
            }
            for j in i+1..=s.len() {
                if set.contains(&s[i..j]) {
                    continue;
                }
                set.insert(s[i..j].to_string());
                backtrack(j, set, ans, s);
                set.remove(&(s[i..j].to_string()));
            }
        }
        let mut ans = 0;
        backtrack(0, &mut set, &mut ans, &s);
        ans as _
    }
}

