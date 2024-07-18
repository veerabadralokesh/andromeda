static modulo: i64 = 1_000_000_007;
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0_i64; n.max(3)+1];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 5;
        for i in 4..=n {
            dp[i] = ((dp[i-1] << 1) + dp[i-3]) % modulo;
        }
        (dp[n]) as _
    }
}

