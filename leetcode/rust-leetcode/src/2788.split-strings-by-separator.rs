impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut ans = vec![];
        for word in words.iter() {
            for split in word.split(separator) {
                if split.len() != 0 {
                    ans.push(split.to_string());
                }
            }
        }
        ans
    }
}

