impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let is_prefix_and_suffix = |i: usize, j: usize| -> bool {
            words[j].len() >= words[i].len() && words[j].starts_with(&words[i]) && words[j].ends_with(&words[i])
        };
        let mut count = 0;
        for i in 0..words.len()-1 {
            for j in i+1..words.len() {
                if is_prefix_and_suffix(i, j) {
                    count += 1;
                }
            }
        }
        count
    }
}

