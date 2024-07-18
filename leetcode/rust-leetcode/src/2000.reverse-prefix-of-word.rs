impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.split_once(ch) {
            Some((pre, post)) => {
                ch.to_string() +
                &pre.chars().rev().collect::<String>() +
                post
            }
            None => word
        }
    }
}

/* */

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match (word.find(ch)) {
            Some(x) => {
                let mut chars = word.chars().collect::<Vec<char>>();
                chars[0..x+1].reverse();
                chars.iter().collect()
            },
            None => {
                word
            }
        }
    }
}
