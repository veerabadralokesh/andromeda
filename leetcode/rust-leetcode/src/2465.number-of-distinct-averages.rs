use std::collections::HashSet;
impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let (mut left, mut right) = (0, nums.len()-1);
        let mut set = HashSet::new();
        while left < right {
            set.insert((nums[left]+nums[right]) * 10/2);
            left += 1;
            right -= 1;
        }
        set.len() as i32
    }
}

