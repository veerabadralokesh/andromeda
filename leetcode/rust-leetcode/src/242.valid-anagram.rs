impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return false;}
        let mut counts = [0;26];
        for b in s.into_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        for b in t.into_bytes() {
            if counts[(b-b'a') as usize] == 0 {
                return false;
            }
            counts[(b-b'a') as usize] -= 1;
        }
        true
    }
}

