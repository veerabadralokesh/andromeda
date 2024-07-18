impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort();
        cost.reverse();
        let mut ans = 0;
        for i in 0..cost.len() {
            if (i+1) % 3 != 0 {
                ans += cost[i];
            }
        }
        ans
    }
}

