impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        if words.len() == 1 {
            return true;
        }
        let mut new_order = [0; 26];
        for (i, c) in order.as_bytes().into_iter().enumerate() {
            new_order[(c - b'a') as usize] = (i as u8 + b'a');
        }
        let alien_words = words.into_iter().map(
            |word| word.into_bytes().iter().map(|&b| new_order[(b-b'a') as usize] as char).collect::<String>()
        ).collect::<Vec<_>>();
        for i in 1..alien_words.len() {
            if alien_words[i] < alien_words[i-1] {
                return false;
            }
        }
        true
    }
}

