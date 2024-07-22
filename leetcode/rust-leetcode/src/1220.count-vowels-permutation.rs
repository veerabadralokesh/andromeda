impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let k_mod = 1_000_000_007_i64;
        let n = n as usize;
        let mut dp = vec![vec![0; 5]; n];
        dp[0] = vec![1, 1, 1, 1, 1];
        for i in 1..n {
            // a
            dp[i][0] = (dp[i-1][1] + dp[i-1][2] + dp[i-1][4]) % k_mod;
            // e
            dp[i][1] = (dp[i-1][0] + dp[i-1][2]) % k_mod;
            // i
            dp[i][2] = (dp[i-1][1] + dp[i-1][3]) % k_mod;
            // o
            dp[i][3] = dp[i-1][2];
            // u
            dp[i][4] = (dp[i-1][2] + dp[i-1][3]) % k_mod;
        }
        ((dp[n-1].iter().sum::<i64>()) % k_mod) as _
    }
}

