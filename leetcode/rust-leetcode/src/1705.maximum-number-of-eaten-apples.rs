use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap:BinaryHeap<(Reverse<usize>, i32)> = BinaryHeap::new();
        let mut ndays = 0;
        let mut i = 0;
        while i < apples.len() || !heap.is_empty() {
            while !heap.is_empty() && heap.peek().unwrap().0.0 <= i {
                heap.pop();
            }
            if i < apples.len() && apples[i] > 0 {
                heap.push((Reverse(i + days[i] as usize), apples[i]));
            }
            if !heap.is_empty() {
                let apple_box = heap.pop().unwrap();
                if apple_box.1 > 1 {
                    heap.push((apple_box.0, apple_box.1-1));
                }
                ndays += 1;
            }
            i += 1;
        }
        ndays
    }
}


