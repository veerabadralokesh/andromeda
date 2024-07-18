use std::collections::*;
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::with_capacity(gifts.len());
        for &g in gifts.iter() {
            heap.push(g as i64);
        }
        let mut sum = 0;
        for _ in 0..k {
            if let Some(x) = heap.pop() {
                if x == 1 {
                    heap.push(x);
                    break;
                }
                heap.push(f64::sqrt(x as f64) as i64);
            }
        }
        heap.iter().sum::<i64>()
    }
}

