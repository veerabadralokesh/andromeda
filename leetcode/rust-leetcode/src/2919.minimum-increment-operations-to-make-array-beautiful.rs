use std::cmp::{min,max};
impl Solution {
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let nums = nums.iter().map(|&n| n as i64).collect::<Vec<i64>>();
        let (mut p1, mut p2, mut p3, mut dp) = (0, 0, 0, 0);
        for n in nums {
            dp = min(p1, min(p2, p3)) + max(0, k - n);
            p3 = p2;
            p2 = p1;
            p1 = dp;
        }
        min(p1, min(p2, p3))
    }
}

