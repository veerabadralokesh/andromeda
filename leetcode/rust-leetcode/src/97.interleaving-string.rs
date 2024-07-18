impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m + n != s3.len() {
            return false;
        }
        if m + n == 0 && s3.len() == 0 {
            return true;
        }
        let mut dp = vec![vec![false; n+1]; m+1];
        dp[0][0] = true;
        for i in 1..m+1 {
            dp[i][0] = dp[i-1][0] && s1[i-1..i] == s3[i-1..i];
        }
        for j in 1..n+1 {
            dp[0][j] = dp[0][j-1] && s2[j-1..j] == s3[j-1..j];
        }

        let mut ij = 0;
        for i in 1..m+1 {
            for j in 1..n+1 {
                ij = i + j;
                dp[i][j] = (dp[i-1][j] && s1[i-1..i] == s3[ij-1..ij]) || (
                    dp[i][j-1] && s2[j-1..j] == s3[ij-1..ij]
                );
            }
        }

        // for row in dp.iter() {
        //     println!("{:?}", row);
        // }

        dp[m][n]
    }
}

/* */

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m + n != s3.len() {
            return false;
        }
        if m + n == 0 && s3.len() == 0 {
            return true;
        }
        let mut dp = vec![false; n+1];
        let mut ij = 0;

        for i in 0..m+1 {
            for j in 0..n+1 {
                if i == 0 && j == 0 {
                    dp[j] = true;
                } else if i == 0 {
                    dp[j] = dp[j-1] && s2[j-1..j] == s3[j-1..j];
                } else if j == 0 {
                    dp[j] = dp[j] && s1[i-1..i] == s3[i-1..i];
                } else {
                    ij = i + j;
                    dp[j] = (dp[j] && s1[i-1..i] == s3[ij-1..ij]) || (
                        dp[j-1] && s2[j-1..j] == s3[ij-1..ij]
                    );
                }
            }
        }

        // for row in dp.iter() {
        //     println!("{:?}", row);
        // }

        dp[n]
    }
}

