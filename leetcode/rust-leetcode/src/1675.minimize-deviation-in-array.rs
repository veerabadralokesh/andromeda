use std::collections::BinaryHeap;
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut min = i32::MAX;
        for &n in nums.iter() {
            let even = if n & 1 == 0 {n} else {n << 1};
            heap.push(even);
            min = min.min(even);
        }
        let mut min_deviation = i32::MAX;
        while *heap.peek().unwrap() & 1 == 0 {
            let max = heap.pop().unwrap();
            min_deviation = min_deviation.min(max - min);
            min = min.min(max/2);
            heap.push(max/2);
        }
        min_deviation.min(*heap.peek().unwrap() - min)
    }
}

