impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map1 = std::collections::HashMap::new();
        for w in s1.split_whitespace() {
            *map1.entry(w).or_insert(0) += 1;
        }
        for w in s2.split_whitespace() {
            *map1.entry(w).or_insert(0) += 1;
        }
        let mut ans = Vec::new();
        for (k, v) in map1.into_iter() {
            if v == 1 {
                ans.push(k.to_string());
            }
        }
        ans
    }
}

