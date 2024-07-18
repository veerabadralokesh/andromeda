impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut reversed = String::with_capacity(s.len());
        for w in s.split(" ") {
            if reversed.len() != 0 {
                reversed.push_str(" ");
            }
            reversed.push_str(&w.chars().rev().collect::<String>());   
        }
        reversed
    }
}