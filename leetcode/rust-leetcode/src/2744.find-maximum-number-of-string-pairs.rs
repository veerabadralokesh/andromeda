use std::collections::HashSet;
impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut ans = 0i32;
        let mut hs:HashSet<String> = HashSet::new();
        
        for word in words.iter() {
            if hs.contains(word) {
                ans += 1;
                hs.remove(word);
            } else {
                let rword = word.chars().rev().collect::<String>();
                hs.insert(rword);
            }
        }
        ans
    }
}
