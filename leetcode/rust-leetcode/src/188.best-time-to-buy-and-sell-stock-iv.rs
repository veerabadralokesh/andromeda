use std::cmp::max;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        if k >= prices.len()/2 {
            let (mut sell, mut hold) = (0, i32::MIN);

            for price in prices {
                sell = max(sell, hold+price);
                hold = max(hold, sell-price);
            }
            return sell;
        }

        let mut sell = vec![0; k+1];
        let mut hold = vec![i32::MIN; k+1];

        for price in prices {
            for i in (1..=k).rev() {
                sell[i] = max(sell[i], hold[i] + price);
                hold[i] = max(hold[i], sell[i-1] - price);
            }
        }
        // println!("{:?}", sell);
        sell[k]
    }
}

