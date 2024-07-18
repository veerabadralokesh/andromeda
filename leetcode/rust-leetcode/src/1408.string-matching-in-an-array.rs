impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        words.sort_by_key(|w| w.len());
        let mut ans = Vec::with_capacity(words.len());
        for i in 0..words.len()-1 {
            for j in (i+1..words.len()).rev() {
                if words[j].contains(&words[i]) {
                    ans.push(words[i].to_string());
                    break;
                }
            }
        }
        ans
    }
}

