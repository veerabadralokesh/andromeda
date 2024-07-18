use std::collections::HashMap;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut map:HashMap<i32, i32> = HashMap::with_capacity(3);
        for n in nums.iter() {
            let count = map.entry(*n).or_insert(0);
            *count += 1;
        }
        for i in 0..nums.len() {
            for j in 0..3 {
                let count = map.get(&j).copied().unwrap_or(0);
                if count > 0 {
                    nums[i] = j;
                    map.insert(j, count-1);
                    break;
                }
            }
        }
        // nums.sort();
    }
}

/* */

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let (mut left, mut right) = (0, nums.len()-1);
        let mut i = 0;
        while i <= right && right > 0 {
            if nums[i] == 0 && i != left {
                nums.swap(i, left);
                left += 1;
            } else if nums[i] == 2 {
                nums.swap(i, right);
                right -= 1;
            } else {
                i += 1;
            }
        }
    }
}

