use std::cmp::Ordering::{Less, Equal, Greater};
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let k_mod = 1_000_000_007_i64;
        let s = s.as_bytes();
        let l = s.len();
        let mut dp = vec![vec![0; l]; l];
        let (mut left, mut right) = (0, 0);
        for i in (0..l).rev() {
            dp[i][i] = 1;
            for j in i+1..l {
                if s[i] == s[j] {
                    left = i + 1;
                    right = j - 1;
                    while left <= right && s[left] != s[i] {
                        left += 1;
                    }
                    while left <= right && s[right] != s[i] {
                        right -= 1;
                    }
                    match left.cmp(&right) {
                        Less => {
                            dp[i][j] = 2 * dp[i + 1][j - 1] - dp[left + 1][right - 1];
                        },
                        Equal => {
                            dp[i][j] = 2 * dp[i+1][j-1] + 1;
                        },
                        Greater => {
                            dp[i][j] = 2 * dp[i+1][j-1] + 2;
                        }
                    }
                } else {
                    dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                }
                dp[i][j] = (dp[i][j] + k_mod) % k_mod;
            }
        }
        // for row in dp.iter() {
        //     println!("{:?}", row);
        // }
        dp[0][l-1] as _
    }
}

