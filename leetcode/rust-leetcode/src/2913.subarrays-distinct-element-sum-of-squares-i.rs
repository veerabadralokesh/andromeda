use std::collections::HashSet;
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..nums.len() {
            let mut set = HashSet::new();
            for j in i..nums.len() {
                set.insert(nums[j]);
                sum += set.len().pow(2) as i32;
            }
        }
        sum
    }
}

