use std::cmp::max;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let rems = nums.into_iter().map(|n| n as usize % k).collect::<Vec<_>>();
        let (mut max_len, mut rem) = (0, 0);
        let mut dp = vec![vec![0; k]; k];
        for &rem in rems.iter() {
            for i in 0..k {
                dp[rem][i] = max(dp[rem][i], dp[i][rem]+1);
                max_len = max(max_len, dp[rem][i]);
            }
        }
        max_len
    }
}

