impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let word = word.to_lowercase();
        (
            !word.chars().any(|c| !c.is_alphabetic() && !c.is_ascii_digit())
            &&
            word.matches(|c: char| "aeiou".contains(c)).count() >= 1
            &&
            word.matches(|c: char| c.is_alphabetic() && !"aeiou".contains(c)).count() >= 1
        )
    }
}

