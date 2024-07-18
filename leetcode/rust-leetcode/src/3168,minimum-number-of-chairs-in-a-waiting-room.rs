impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut max = 0;
        let mut curr = 0;
        for b in s.into_bytes() {
            if b'E' == b {
                curr += 1;
                max = max.max(curr);
            } else {
                curr -= 1;
            }
        }
        max
    }
}

