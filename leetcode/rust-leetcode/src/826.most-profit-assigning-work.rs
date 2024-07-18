impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
        worker.sort_by_key(|&w| w);
        let mut job_profits = difficulty.into_iter().zip(profit).collect::<Vec<(i32,i32)>>();
        job_profits.sort_by_key(|jp| jp.0);
        let n = job_profits.len();
        for i in 1..n {
            job_profits[i].1 = job_profits[i].1.max(job_profits[i-1].1)
        }
        let mut i = 0;
        let mut max_profit = 0;
        for &w in worker.iter() {
            while i < n && job_profits[i].0 <= w {
                i += 1;
            }
            if i < n+1 && i > 0 {
                max_profit += job_profits[i-1].1;
            }
        }
        max_profit
    }
}

