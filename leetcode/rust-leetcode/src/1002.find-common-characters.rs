impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut counts = vec![[0; 26]; words.len()];
        let mut common = [i32::MAX; 26];
        for (i, word) in words.iter().enumerate() {
            for b in word.clone().into_bytes() {
                counts[i][(b-b'a') as usize] += 1;
            }
            for j in 0..26 {
                common[j] = common[j].min(counts[i][j]);
            }
        }
        let mut ans = Vec::new();
        for i in 0..26 {
            for _ in 0..common[i] {
                ans.push(String::from((i as u8 + b'a') as char));
            }
        }
        ans
    }
}

