use std::cmp::max;
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let l = nums.len();
        let mut dp = vec![(1, 0); l];
        for i in 1..l {
            dp[i].1 = i;
            for j in (0..i).rev() {
                if nums[i] % nums[j] == 0 && dp[i].0 < dp[j].0 + 1 {
                    dp[i].0 = max(dp[i].0, dp[j].0 + 1);
                    dp[i].1 = j;
                }
            }
        }
        let (mut max_size, mut max_idx) = (0, 0);
        for i in 0..l {
            if max_size < dp[i].0 {
                max_idx = i;
                max_size = dp[i].0;
            }
        }
        let mut ans = Vec::with_capacity(max_size);
        ans.push(nums[max_idx]);
        while max_idx != dp[max_idx].1 {
            max_idx = dp[max_idx].1;
            ans.push(nums[max_idx]);
        }
        ans
    }
}

