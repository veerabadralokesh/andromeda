use std::collections::*;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(|n| n as i64));
        let div_ceil = |num: i64| -> i64 {
            (num / 3) + (num % 3 != 0) as i64
        };
        let mut ans = 0;
        for _ in 0..k {
            if let Some(x) = heap.pop() {
                ans += x;
                heap.push(div_ceil(x));
            }
        }
        ans
    }
}

/* */

use std::collections::*;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(|n| n as i64));
        let mut ans = 0;
        for _ in 0..k {
            if let Some(x) = heap.pop() {
                ans += x;
                heap.push((x + 2)/3);
            }
        }
        ans
    }
}

