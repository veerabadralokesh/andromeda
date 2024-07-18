use std::collections::BTreeSet;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut min_queue = Vec::with_capacity(nums.len());
        let mut max_queue = Vec::with_capacity(nums.len());
        let mut i = 0;
        for n in nums.iter() {
            while !min_queue.is_empty() && n < min_queue.last().unwrap() {
                min_queue.pop();
            }
            while !max_queue.is_empty() && n > max_queue.last().unwrap() {
                max_queue.pop();
            }
            min_queue.push(*n);
            max_queue.push(*n);
            if max_queue[0] - min_queue[0] > limit {
                if min_queue[0] == nums[i] {
                    min_queue.remove(0);
                }
                if max_queue[0] == nums[i] {
                    max_queue.remove(0);
                }
                i += 1;
            }
        }
        (nums.len() - i) as i32
    }
}

