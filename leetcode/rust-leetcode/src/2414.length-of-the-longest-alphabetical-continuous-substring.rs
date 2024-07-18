impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut count = 1;
        let mut ans = 1;
        let s = s.into_bytes();
        for i in 1..s.len() {
            if s[i]-s[i-1] == 1 {
                count += 1;
                ans = ans.max(count);
            } else {
                count = 1;
            }
        }
        ans
    }
}
