use std::collections::*;
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(piles.into_iter());
        for _ in 0..k {
            if let Some(x) = heap.pop() {
                if x == 1 {
                    return 1 + heap.iter().sum::<i32>();
                }
                heap.push((x - (x >> 1)));
            }
        }
        heap.iter().sum::<i32>()
    }
}

