impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.to_vec();
        sorted.sort_unstable();
        let (mut start, mut end) = (0, 0);
        for i in 0..nums.len() {
            if nums[i] != sorted[i] {
                start = i;
                break;
            }
        }
        for i in (0..nums.len()).rev() {
            if nums[i] != sorted[i] {
                end = i;
                break;
            }
        }
        if start == end {
            0
        } else {
            (end + 1 - start) as _
        }
    }
}

/* */

// LEARN

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut prev_max = i32::MIN;
        let mut prev_min = i32::MAX;
        let mut right_bound : i32 = -1;
        let mut left_bound : i32 = -1;
        for (i, num) in nums.iter().rev().enumerate() {
            if *num > prev_min {
                left_bound = nums.len() as i32 - i as i32 - 1;
            }
            prev_min = prev_min.min(*num);
        }
        for (i, num) in nums.iter().enumerate() {
            if *num < prev_max {
                right_bound = i as i32;
            }
            prev_max = prev_max.max(*num);
        }
        if right_bound == -1 && left_bound == -1 {
            return 0;
        }
        right_bound as i32 - left_bound as i32 + 1
    }
}

