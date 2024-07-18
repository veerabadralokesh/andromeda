impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        if words.len() < 2 {return true;}
        let mut counts = [0; 26];
        for word in words.iter() {
            for b in word.as_bytes() {
                counts[(b - b'a') as usize] += 1;
            }
        }
        let mut count = 0;
        for i in 0..26 {
            if counts[i] % words.len() != 0 {
                return false;
            }
        }
        true
    }
}

