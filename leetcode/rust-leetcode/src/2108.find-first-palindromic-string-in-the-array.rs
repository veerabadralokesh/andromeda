
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let mut first_palindrome = String::new();
        let mut palindrome: bool = false;
        for word in words.iter() {
            let half = word.len()/2;
            palindrome = word.bytes().take(half).eq(word.bytes().rev().take(half));
            if palindrome {
                return word.to_string();
            }
        }
        first_palindrome
    }
}
impl Solution2 {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words.iter() {
            if word.bytes().eq(word.bytes().rev()) {
                return word.to_string();
            }
        }
        String::new()
    }
}

