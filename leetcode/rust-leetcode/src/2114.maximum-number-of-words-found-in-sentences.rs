// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max_count: i32 = 0;
        for sentence in sentences.iter() {
            let mut sentence_count: i32 = 0;
            for word in sentence.split_whitespace() {
                sentence_count += 1;
            }
            if (sentence_count > max_count) {
                max_count = sentence_count;
            }
        }
        return max_count;
    }
}

impl Solution2 {
    pub fn most_words_found(mut sentences: Vec<String>) -> i32 {
        sentences.iter().map(|s| s.split_whitespace().count()).max().unwrap() as i32
    }
}