use std::cmp::{max,min};
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&n| n as i128).collect::<Vec<_>>();
        let n = nums.len();
        let mut dp_max = nums.to_vec();
        let mut dp_min = nums.to_vec();
        for i in 1..n {
            dp_max[i] = nums[i].max(max(dp_max[i-1] * nums[i], dp_min[i-1] * nums[i]));
            dp_min[i] = nums[i].min(min(dp_max[i-1] * nums[i], dp_min[i-1] * nums[i]));
        }
        dp_max.into_iter().max().unwrap() as _
    }
}

