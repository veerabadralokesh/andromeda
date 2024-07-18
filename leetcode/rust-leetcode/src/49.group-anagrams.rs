use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let mut counts = [0; 26];
            for b in s.clone().into_bytes() {
                counts[(b-b'a') as usize] += 1;
            }
            map.entry(counts).or_insert(vec![]).push(s);
        }
        let mut ans = vec![];
        for (k, v) in map.into_iter() {
            ans.push(v);
        }
        ans
    }
}