use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        fn dp(nums: &Vec<i32>, start: usize, end: usize, target: i32, map: &mut HashMap<(usize,usize,i32), i32>) -> i32 {
            if start >= end {
                return 0;
            }
            if map.contains_key(&(start, end, target)) {
                return *map.get(&(start, end, target)).unwrap();
            }
            let delete_first_two = if nums[start] + nums[start+1] == target {
                1 + dp(nums, start+2, end, target, map)
            } else {
                0
            };
            let delete_last_two = if nums[end] + nums[end.saturating_sub(1)] == target {
                1 + dp(nums, start, end.saturating_sub(2), target, map)
            } else {
                0
            };
            let delete_first_last = if nums[start] + nums[end] == target {
                1 + dp(nums, start+1, end.saturating_sub(1), target, map)
            } else {
                0
            };
            let return_val = max(
                max(delete_first_two, delete_last_two),
                delete_first_last,
            );
            map.insert((start, end, target), return_val);
            return_val
        }
        let n = nums.len();
        let mut map = HashMap::new();
        max(
            max (
                dp(&nums, 0, n-1, nums[0]+nums[1], &mut map),
                dp(&nums, 0, n-1, nums[n-2]+nums[n-1], &mut map)
            ),
            dp(&nums, 0, n-1, nums[0]+nums[n-1], &mut map)
        )
    }
}

