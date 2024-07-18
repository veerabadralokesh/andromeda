impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut is_rotated = false;
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {
                if is_rotated {
                    return false;
                }
                is_rotated = true;
            }
        }
        !is_rotated || *nums.last().unwrap() <= nums[0]
    }
}

