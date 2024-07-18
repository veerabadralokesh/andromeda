impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut ans = 1;
        let sb = s.into_bytes();
        let mut count = 1;
        for i in 1..sb.len() {
            if sb[i] == sb[i-1] {
                count += 1;
                ans = ans.max(count);
            } else {
                count = 1;
            }
        }
        ans
    }
}

