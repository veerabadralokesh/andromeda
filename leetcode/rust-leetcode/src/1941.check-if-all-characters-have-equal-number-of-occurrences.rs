impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        if s.len() == 1 {return true;}
        let mut counts = [0; 26];
        for &b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut count = 0;
        for c in counts.into_iter() {
            if c != 0 && count == 0 {
                count = c;
            } else if c > 0 && c != count {
                return false;
            }
        }
        true
    }
}

