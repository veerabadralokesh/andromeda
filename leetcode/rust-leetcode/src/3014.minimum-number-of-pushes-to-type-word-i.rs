impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut ans = 0;
        let mut l = word.len();
        let mut mult = 1;
        while l > 0 {
            ans += (l.min(8) * mult);
            mult += 1;
            l = l.saturating_sub(8);
        }
        ans as _
    }
}

