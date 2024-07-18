use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut profit = 0;
        let mut holding = i32::MIN;
        for &price in prices.iter() {
            profit = max(profit, holding + price);
            holding = max(holding, profit - price - fee);
        }
        profit
    }
}

/* */

// Doesn't scale

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![0;n+1];n+1];
        for i in 0..n {
            for j in 0..n {
                if i < j && prices[j] - prices[i] > fee {
                    dp[i+1][j+1] = max(
                        dp[i+1][i] + (prices[j] - prices[i] - fee),
                        dp[i][j+1]
                    );
                } else {
                    dp[i+1][j+1] = max(dp[i][j+1], dp[i+1][j]);
                }
            }
        }
        // for row in dp.iter() {
        //     println!("{:?}", row);
        // }
        dp[n][n]
    }
}

