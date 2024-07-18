impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut increment = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i-1] {
                increment += (nums[i-1] - nums[i]) + 1;
                nums[i] = nums[i-1] + 1;
            }
        }
        increment
    }
}

