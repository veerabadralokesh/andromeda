impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut counts = std::collections::HashMap::new();
        for word in words.iter() {
            let mut c = counts.entry(word).or_insert(0);
            *c += 1;
            if *c == 1 {
                ans.push(word.clone());
            }
        }
        ans.sort_by_key(|word| (-*counts.get(&word).unwrap(), word.clone()));
        ans[..k as usize].to_vec()
    }
}

