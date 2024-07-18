impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut ccounts = [false; 26];
        let mut scounts = [false; 26];
        for b in word.into_bytes() {
            if b > b'Z' {
                scounts[(b - b'a') as usize] = true;
            } else {
                ccounts[(b - b'A') as usize] = true;
            }
        }
        (0..26).filter(|&i| scounts[i] && ccounts[i]).count() as _
    }
}

