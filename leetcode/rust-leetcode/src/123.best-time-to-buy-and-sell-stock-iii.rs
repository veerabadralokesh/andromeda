use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut hold1, mut sell1, mut hold2, mut sell2) = (i32::MIN, 0, i32::MIN, 0);

        for price in prices {
            // println!("{price} {sell2} {hold2} {sell1} {hold1}");
            sell2 = max(sell2, hold2 + price);
            hold2 = max(hold2, sell1 - price);
            sell1 = max(sell1, hold1 + price);
            hold1 = max(hold1, -price);
            // println!("{price} {sell2} {hold2} {sell1} {hold1}");
        }

        sell2
    }
}

