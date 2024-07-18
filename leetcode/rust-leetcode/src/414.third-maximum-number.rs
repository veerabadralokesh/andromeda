use std::collections::HashSet;
impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        let mut set:HashSet<_> = nums.into_iter().collect();
        let mut nums:Vec<_> = set.into_iter().collect();
        nums.sort();
        if nums.len() > 2 {
            nums[nums.len()-3]
        } else {
            nums[nums.len()-1]
        }
    }
}

/* */

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max = None;
        let mut middle = None;
        let mut min = None;
        let mut length = 0;

        for num in nums {
            if Some(num) == max || Some(num) == middle || Some(num) == min {
                continue;
            }
            if Some(num) > max {
                (min, middle, max) = (middle, max, Some(num));
            } else if Some(num) > middle {
                (min, middle) = (middle, Some(num));
            } else if Some(num) > min {
                min = Some(num);
            }
            length += 1;
        }

        if length < 3 { max.unwrap() } else { min.unwrap() }
    }
}
