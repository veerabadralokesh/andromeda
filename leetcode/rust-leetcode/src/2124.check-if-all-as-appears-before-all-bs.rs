impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut b_found = false;
        for b in s.into_bytes() {
            if b == b'b' {
                b_found = true;
            }
            if b_found && b == b'a' {
                return false;
            }
        }
        true
    }
}

