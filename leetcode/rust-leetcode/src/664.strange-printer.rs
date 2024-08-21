// Bottom Up

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![vec![n as i32; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }
        for j in 0..n {
            for i in (0..=j).rev() {
                for k in i..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k+1][j] - (s[k..k+1] == s[j..j+1]) as i32);
                }
            }
        }
        dp[0][n-1]
    }
}

/* */

// Top Down

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![vec![0 as i32; n]; n];
        Self::helper(&s, 0, n-1, &mut dp)
    }

    fn helper(s: &String, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i > j {
            return 0;
        }
        if dp[i][j] == 0 {
            dp[i][j] = Self::helper(s, i+1, j, dp) + 1;
            for k in i+1..=j {
                if s[i..i+1] == s[k..k+1] {
                    dp[i][j] = dp[i][j].min(
                        Self::helper(s, i, k-1, dp) + Self::helper(s, k+1, j, dp)
                    );
                }
            }
        }
        dp[i][j]
    }
}

