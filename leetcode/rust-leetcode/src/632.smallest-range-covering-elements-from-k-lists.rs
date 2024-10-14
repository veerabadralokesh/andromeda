use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut minHeap = BinaryHeap::with_capacity(n);

        let (mut mx, mut mn) = (i32::MIN, i32::MAX);
        
        
        for (i, list) in nums.iter().enumerate() {
            let x = list[0];
            minHeap.push((Reverse(x), i, 0));
            mx = mx.max(x);
            mn = mn.min(x);
        }

        let (mut minRange, mut maxRange) = (mn, mx);

        while minHeap.len() == n {
            let (_, i, j) = minHeap.pop().unwrap();
            if j + 1 < nums[i].len() {
                minHeap.push((Reverse(nums[i][j+1]), i, j+1));
                mx = mx.max(nums[i][j+1]);
                mn = minHeap.peek().unwrap().0.0;
                if mx - mn < maxRange - minRange {
                    minRange = mn;
                    maxRange = mx;
                }
            }
        }
        vec![minRange, maxRange]
    }
}

