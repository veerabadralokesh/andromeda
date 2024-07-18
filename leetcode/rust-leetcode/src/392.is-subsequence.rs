
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        if t.len() == 0 {
            return false;
        }
        let mut i:usize = 0;
        let mut j:usize = 0;
        let s = s.into_bytes();
        let t = t.into_bytes();
        while j < t.len() {
            if s[i] == t[j] {
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
            j += 1;
        }
        false
    }
}
