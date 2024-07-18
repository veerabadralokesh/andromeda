impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total = 0i32;
        let mut minp = prices[0];
        let mut maxp = prices[0];
        for p in prices {
            if p < maxp {
                total += (maxp - minp);
                minp = p;
                maxp = p;
            } else if p > maxp {
                maxp = p;
            }
        }
        if maxp > minp {
            total += (maxp - minp);
        }
        total
    }
}

/*
*/

use std::cmp;

impl Solution2 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n: usize = prices.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; n+1];
        dp[n][0] = 0;
        dp[n][1] = 0;
        for i in (0..n).rev() {
            for j in 0..2 {
                if j == 1 {
                    dp[i][j] = cmp::max(-prices[i] + dp[i+1][0], dp[i+1][1]);
                } else {
                    dp[i][j] = cmp::max(prices[i] + dp[i+1][1], dp[i+1][0]);
                }
            }
        }
        dp[0][1]
    }
}