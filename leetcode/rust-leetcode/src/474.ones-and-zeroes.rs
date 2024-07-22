impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n, mut z, mut o) = (m as usize, n as usize, 0, 0);
        let mut dp = vec![vec![0; n+1]; m+1];
        for s in strs.iter() {
            z = s.as_bytes().iter().filter(|&b| *b == b'0').count();
            o = s.len() - z;
            for i in (z..=m).rev() {
                for j in (o..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i-z][j-o] + 1);
                }
            }
        }
        dp[m][n] as _
    }
}

