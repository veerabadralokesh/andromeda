impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        100 * (s.clone().chars().filter(|&c| c == letter).count() as i32) / (s.len() as i32)
    }
}

