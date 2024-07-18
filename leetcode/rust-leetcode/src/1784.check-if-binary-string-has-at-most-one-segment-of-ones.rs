impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut sb = s.into_bytes();
        let mut count = sb[0] - b'0';
        for i in 1..sb.len() {
            if sb[i] == b'1' && sb[i-1] == b'0' {
                count += 1;
            }
        }
        count < 2
    }
}

