impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let mut dp = vec![vec![(0,0); n+1]; m+1];
        // let mut map = HashMap::new();
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if s[i..i+1] == t[j..j+1] {
                    dp[i+1][j+1].0 = 1 + dp[i][j].0;
                    dp[i+1][j+1].1 = dp[i][j].1;
                    ans += dp[i+1][j+1].1;
                } else {
                    ans += 1 + dp[i][j].0;
                    dp[i+1][j+1].1 = dp[i][j].0 + 1;
                }
                // print!("{ans}, ");
            }
            // println!("\n{ans}");
        }
        // for row in dp.iter() {
        //     println!("{:?}", row);
        // }
        ans as i32
    }
}

