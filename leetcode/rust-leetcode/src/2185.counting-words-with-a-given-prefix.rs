impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let pref = pref.as_str();
        words.iter().filter(|w| w.starts_with(pref)).count() as i32
    }
}

/*
*/

// LEARN

impl Solution2 {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|w| w.len() >= pref.len()).map(|w| if pref == w[0..pref.len()] {1} else {0}).sum()
    }
}
