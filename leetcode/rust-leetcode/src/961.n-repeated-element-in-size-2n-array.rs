impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 0..(nums.len()-2) {
            if nums[i] == nums[i+1] || nums[i] == nums[i+2] {
                return nums[i];
            }
        }
        *nums.last().unwrap()
    }
}

/* */

use std::collections::HashSet;
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in nums.into_iter() {
            if set.contains(&n) {
                return n;
            }
            set.insert(n);
        }
        0
    }
}
