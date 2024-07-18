impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut ans = cost.clone();
        for i in 2..ans.len() {
            ans[i] += ans[i-1].min(ans[i-2]);
        }
        // println!("{:?}", ans);
        ans[ans.len()-1].min(ans[ans.len()-2])
    }
}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0;n+1];
        dp[0] = 0;
        dp[1] = 0;
        for i in 2..=n {
            dp[i] = std::cmp::min(dp[i-1] + cost[i-1], dp[i-2] + cost[i-2]);
        }
        dp[n]
    }
}
