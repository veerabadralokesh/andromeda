use std::cmp::{min, max};
impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let high = *nums.last().unwrap();
        let low = nums[0];
        let (mut maxi, mut mini) = (0, 0);
        let mut ans = high - low;
        for i in 0..(nums.len()-1) {
            maxi = max(high - k, nums[i] + k);
            mini = min(low + k, nums[i+1] - k);
            ans = min(ans, maxi - mini);
        }
        ans
    }
}

