use std::cmp::max;
impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return *nums.iter().max().unwrap();
        }
        nums[2] += nums[0];
        let mut m = max(max(nums[0], nums[1]), nums[2]);
        for i in 3..nums.len() {
            nums[i] += max(nums[i-2], nums[i-3]);
            m = m.max(nums[i]);
        }
        m
    }
}
