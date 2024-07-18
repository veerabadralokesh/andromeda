impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        // words.iter().map(|c| c.chars().next().unwrap()).collect::<String>() == s
        words.iter().map(|c| c.chars().nth(0).unwrap()).collect::<String>() == s
    }
}