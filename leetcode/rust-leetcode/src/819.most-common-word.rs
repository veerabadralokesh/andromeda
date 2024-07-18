use std::collections::{HashSet,HashMap};
// static punctuation: &[char] = &['!', '?', '\'', ',', ';', '.'];
static punctuation: &[char] = &['!', '?', '\'', ',', ';', '.', ' '];
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned:HashSet<_> = banned.into_iter().collect();
        let mut counter = HashMap::new();
        let mut ans = String::new();
        let mut count = 0;
        // for word in paragraph.split_whitespace() {
        for word in paragraph.rsplit(punctuation) {
            let word = word.trim_matches(punctuation).to_lowercase();
            // println!("{:?}", word);
            if word.is_empty() || banned.contains(&word) {
                continue;
            }
            let mut c = counter.entry(word.clone()).or_insert(0);
            *c += 1;
            if *c > count {
                count = *c;
                ans = String::from(word);
            }
        }
        ans
    }
}

