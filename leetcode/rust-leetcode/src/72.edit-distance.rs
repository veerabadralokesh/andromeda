use std::cmp::min;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.into_bytes();
        let w2 = word2.into_bytes();
        let (m, n) = (w1.len(), w2.len());
        let mut dp = vec![vec![0; n+1]; m+1];
        for j in 0..=n {dp[0][j] = j};
        for i in 0..=m {dp[i][0] = i};
        for i in 0..m {
            for j in 0..n {
                if w1[i] == w2[j] {
                    dp[i+1][j+1] = dp[i][j];
                } else {
                    dp[i+1][j+1] = 1 + min(
                        dp[i][j], min(dp[i+1][j], dp[i][j+1])
                    );
                }
            }
        }
        dp[m][n] as _
    }
}

