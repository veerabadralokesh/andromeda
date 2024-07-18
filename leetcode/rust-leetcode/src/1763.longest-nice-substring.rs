use std::collections::HashSet;
impl Solution {
    pub fn longest_nice_substring(ss: String) -> String {
        if ss.len() < 2 {
            return String::new();
        }
        let s:Vec<_> = ss.clone().chars().collect();
        let set:HashSet<_> = s.iter().collect();
        for (i, c) in s.iter().enumerate() {
            let c = if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            };
            if !set.contains(&c) {
                let prefix = Self::longest_nice_substring(s[..i].iter().collect());
                let suffix = Self::longest_nice_substring(s[i+1..].iter().collect());
                if prefix.len() < suffix.len() {
                    return suffix;
                } else {
                    return prefix;
                }
            }
        }
        ss
    }
}

