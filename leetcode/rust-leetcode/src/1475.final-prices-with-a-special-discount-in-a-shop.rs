impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ans = prices.to_vec();
        let mut stack = Vec::new();
        for i in 0..prices.len() {
            while !stack.is_empty() && prices[*stack.last().unwrap()] >= prices[i] {
                ans[stack.pop().unwrap()] -= prices[i];
            }
            stack.push(i);
        }
        ans
    }
}
