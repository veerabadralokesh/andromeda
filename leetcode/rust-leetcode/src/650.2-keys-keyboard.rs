impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n+1];
        dp[0] = 0;
        dp[1] = 0;
        for i in 1..=n/2 {
            let mut ops = dp[i]+1;
            for j in (2*i..=n).step_by(i) {
                ops += 1;
                dp[j] = dp[j].min(ops);
            }
        }
        dp[n]
    }
}

