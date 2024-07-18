// LEARN

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let sb = s.into_bytes();
        let tb = t.into_bytes();
        let mut dp = vec![vec![0; n+1]; m+1];
        for i in 0..m+1 {
            dp[i][0] = 1;
        }
        for i in 1..m+1 {
            for j in 1..n+1 {
                if sb[i-1] == tb[j-1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i-1][j];
                }
            }
        }
        dp[m][n]
    }
}

/* */

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let sb = s.into_bytes();
        let tb = t.into_bytes();
        let mut dp = vec![0; n+1];
        dp[0] = 1;
        for i in 1..m+1 {
            for j in (1..n+1).rev() {
                if sb[i-1] == tb[j-1] {
                    dp[j] += dp[j - 1];
                }
            }
        }
        dp[n]
    }
}

