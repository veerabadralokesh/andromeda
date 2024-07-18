use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from_iter(stones.into_iter());
        while heap.len() > 1 {
            if let (Some(y), Some(x)) = (heap.pop(), heap.pop()) {
                if y != x {
                    heap.push(y-x);
                }
            }
        }
        heap.pop().unwrap_or(0)
    }
}

