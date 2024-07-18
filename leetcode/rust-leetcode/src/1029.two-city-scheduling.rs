impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut diffs = costs.iter().enumerate().map(|(i,c)| (c[0]-c[1],i)).collect::<Vec<_>>();
        diffs.sort();
        let mut ans = 0;
        let n = costs.len()/2;
        for i in 0..n {
            ans += costs[diffs[i].1][0];
        }
        for i in n..costs.len() {
            ans += costs[diffs[i].1][1];
        }
        ans
    }
}

/* */

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_by_key(|c| c[0]-c[1]);
        let mut ans = 0;
        let n = costs.len()/2;
        for i in 0..n {
            ans += costs[i][0] + costs[i+n][1];
        }
        ans
    }
}

