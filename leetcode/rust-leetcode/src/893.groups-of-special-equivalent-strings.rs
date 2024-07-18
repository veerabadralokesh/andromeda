use std::collections::HashMap;
impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        let mut wordvecs = HashMap::new();
        for word in words {
            let mut wv = [0; 52];
            for (i, b) in word.bytes().enumerate() {
                if i % 2 == 0 {
                    wv[(b-b'a') as usize] += 1;
                } else {
                    wv[26usize + ((b-b'a') as usize)] += 1;
                }
            }
            *wordvecs.entry(wv.to_vec()).or_insert(0) += 1;
        }
        wordvecs.len() as i32
    }
}
