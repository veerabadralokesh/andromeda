use std::cmp::min;
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let s = sequence.into_bytes();
        let w = word.into_bytes();
        let (m, n) = (s.len(), w.len());
        let mut dp = vec![vec![0; n]; m];
        let mut max_count = 0;
        for i in 0..m {
            for j in 0..n.min(i+1) {
                if i < n {

                    dp[i][j] = if s[i] == w[j] {
                            if i > 0 && j > 0 {min(1, dp[i-1][j-1])} else {1}
                        } else {
                            0
                        };
                } else {
                    dp[i][j] = if s[i] == w[j] {
                            if j > 0 {min(dp[i-n][j]+1, dp[i-1][j-1])} else {dp[i-n][j]+1}
                        } else {
                            0
                        };
                }
            }
            max_count = max_count.max(dp[i][n-1]);
        }
        max_count
    }
}

