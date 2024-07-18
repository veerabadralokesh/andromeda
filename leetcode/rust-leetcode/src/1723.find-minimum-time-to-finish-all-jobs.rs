impl Solution {
    pub fn minimum_time_required(mut jobs: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        jobs.sort();
        jobs.reverse();
        fn backtrack(jobs: &Vec<i32>, job_id: usize, workers: &mut Vec<i32>, ans: &mut i32, bitmask: u32) {
            if job_id == jobs.len() {
                *ans = (*ans).min(*workers.iter().max().unwrap());
                return;
            }
            for worker_id in 0..workers.len() {
                workers[worker_id] += jobs[job_id];
                if workers[worker_id] < *ans {
                    backtrack(jobs, job_id+1, workers, ans, bitmask);
                }
                workers[worker_id] -= jobs[job_id];
                if workers[worker_id] == 0 {
                    return;
                }
            }
        }
        let mut ans = jobs.iter().sum::<i32>();
        let mut workers = vec![0;k];
        backtrack(&jobs, 0, &mut workers, &mut ans, 0);
        ans
    }
}


