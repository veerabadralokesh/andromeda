impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut increasing = None;
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                continue;
            }
            if increasing.is_none() {
                if nums[i] > nums[i-1] {
                    increasing = Some(true);
                } else {
                    increasing = Some(false);
                }
            } else {
                if nums[i] > nums[i-1] {
                    if increasing == Some(false) {
                        return false;
                    }
                } else {
                    if increasing == Some(true) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

