impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let l = s.len();
        let mut dp = vec![vec![0; l]; l];
        for i in (0..l).rev() {
            dp[i][i] = 1;
            for j in i+1..l {
                if s[i..i+1] == s[j..j+1] {
                    dp[i][j] = dp[i+1][j-1] + 2;
                } else {
                    dp[i][j] = std::cmp::max(dp[i+1][j], dp[i][j-1]);
                }
            }
        }
        dp[0][l-1]
    }
}

