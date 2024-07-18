impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut ans = 0;
        let sb = s.as_bytes();
        for i in 2..sb.len() {
            if sb[i] != sb[i-1] && sb[i-1] != sb[i-2] && sb[i] != sb[i-2] {
                ans += 1;
            }
        }
        ans
    }
}

