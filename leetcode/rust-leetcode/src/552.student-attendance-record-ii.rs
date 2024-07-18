static modulo: i64 = 1_000_000_007;
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as i64;
        let mut dp = vec![[0; 3]; 2];
        dp[0][0] = 1;
        for _ in 0..n {
            let prev = dp.to_vec();

            dp[0][0] = (prev[0].iter().sum::<i64>()) % modulo;

            dp[0][1] = prev[0][0];

            dp[0][2] = prev[0][1];

            dp[1][0] = (prev[0].iter().sum::<i64>() + prev[1].iter().sum::<i64>()) % modulo;

            dp[1][1] = prev[1][0];

            dp[1][2] = prev[1][1];
        }
        ((dp[0].iter().sum::<i64>() + dp[1].iter().sum::<i64>()) % modulo) as i32
    }
}

