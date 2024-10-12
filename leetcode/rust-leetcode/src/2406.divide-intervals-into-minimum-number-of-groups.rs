use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut heap:BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        intervals.sort();//sort_by_key(|i| i[0]);
        // println!("{:?}", intervals);
        for i in intervals.iter() {
            if !heap.is_empty() && i[0] > heap.peek().unwrap().0 {
                heap.pop();
            }
            heap.push(Reverse(i[1]));
        }
        heap.len() as _
    }
}

