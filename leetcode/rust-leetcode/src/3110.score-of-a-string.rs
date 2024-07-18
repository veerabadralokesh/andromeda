impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let sb = s.into_bytes();
        let mut ans = 0;
        for i in 1..sb.len() {
            ans += ((sb[i] as i32) - (sb[i-1]) as i32).abs();
        }
        ans
    }
}
