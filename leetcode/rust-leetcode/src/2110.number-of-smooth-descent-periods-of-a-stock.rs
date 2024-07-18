impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 1;
        let mut ans = 0;
        for i in 1..prices.len() {
            if 1 == prices[i-1] - prices[i] {
                count += 1;
            } else {
                ans += (count * (count+1))/2;
                count = 1;
            }
        }
        ans += (count * (count+1))/2;
        ans
    }
}

/* */

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 1;
        let mut ans = 0;
        for i in 1..prices.len() {
            if 1 == prices[i-1] - prices[i] {
                count += 1;
            } else {
                ans += (count * (count+1)) >> 1;
                count = 1;
            }
        }
        ans += (count * (count+1)) >> 1;
        ans
    }
}
