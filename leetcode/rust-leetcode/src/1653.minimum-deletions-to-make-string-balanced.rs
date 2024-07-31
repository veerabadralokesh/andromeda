impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut dp = 0;
        let mut b = 0;
        for c in s.chars() {
            if c == 'a' {
                dp = b.min(dp+1);
            } else {
                b += 1;
            }
        }
        dp
    }
}

