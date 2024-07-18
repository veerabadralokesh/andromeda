use std::cmp::max;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; n+1]; m+1];
        let t1 = text1.into_bytes();
        let t2 = text2.into_bytes();
        for i in 0..m {
            for j in 0..n {
                dp[i+1][j+1] = if t1[i] == t2[j] {
                    1 + dp[i][j]
                } else {
                    max(dp[i][j+1], dp[i+1][j])
                }
            }
        }
        dp[m][n]
    }
}

