use std::collections::HashSet;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(char::is_alphabetic)
            .filter(|s| !s.is_empty())
            .map(|num| num.trim_start_matches('0'))
            .collect::<HashSet<_>>().len() as _
    }
}

