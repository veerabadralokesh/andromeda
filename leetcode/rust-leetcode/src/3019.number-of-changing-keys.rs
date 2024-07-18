impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let slc = s.to_lowercase();
        let sb = slc.into_bytes();
        let mut prevb = sb[0];
        let mut ans = 0i32;
        for b in sb {
            if b != prevb {
                ans += 1;
                prevb = b;
            }
        }
        ans
    }
}
