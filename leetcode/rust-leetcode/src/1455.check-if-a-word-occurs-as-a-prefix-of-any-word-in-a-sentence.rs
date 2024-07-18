impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut i = 1;
        for word in sentence.split_whitespace() {
            if word.starts_with(search_word.as_str()) {
                return i as _;
            }
            i += 1;
        }
        -1
    }
}

