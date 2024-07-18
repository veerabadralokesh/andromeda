use std::cmp::{max,min};
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![0i32; n]; n];
        for i in 0..n {
            for j in (i+1..n).rev() {
                match (j - i - n) % 2 {
                    1 => {
                        dp[i][j] = max(piles[i] + dp[i+1][j], piles[j] + dp[i][j-1]);
                    },
                    0 => {
                        dp[i][j] = min(-piles[i] + dp[i+1][j], -piles[j] + dp[i][j-1]);
                    },
                    _ => unreachable!()
                }
                // break;
            }
            // break;
        }
        // for row in dp.to_vec() {
        //     println!("{:?}", row);
        // }
        dp[0][n-1] > 0
    }
}


impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        true
    }
}

