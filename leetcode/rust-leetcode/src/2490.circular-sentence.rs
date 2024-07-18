impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut words = sentence.split_whitespace().map(|w| w.to_string().into_bytes()).collect::<Vec<_>>();
        words.push(words[0].to_vec());
        for i in 1..words.len() {
            if words[i][0] != *words[i-1].last().unwrap() {
                return false;
            }
        }
        true
    }
}

