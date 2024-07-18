impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut maxprofit:i32 = 0;
        let mut minp = prices[0];
        for p in prices {
            if p < minp {
                minp = p;
            } else if p-minp > maxprofit {
                maxprofit = p-minp;
            }
        }
        maxprofit
    }
}

/* */

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut hold, mut sell) = (i32::MIN, 0);
        for price in prices {
            sell = max(sell, hold + price);
            hold = max(hold, sell - price);
        }
        sell
    }
}


