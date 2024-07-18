impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut count = [0;26];
        for &b in s.as_bytes() {
            count[(b - b'a') as usize] += 1;
            if count[(b - b'a') as usize] > 1 {
                return b as char;
            }
        }
        'a'
    }
}

