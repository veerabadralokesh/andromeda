use std::collections::{HashMap,BinaryHeap};
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &a in arr.iter() {
            *map.entry(a).or_insert(0) += 1;
        }
        let mut l = arr.len();
        let target = l/2;
        let mut heap = BinaryHeap::new();
        for (k, v) in map.into_iter() {
            heap.push(v);
        }
        let mut count = 0;
        while l > target {
            l -= heap.pop().unwrap();
            count += 1;
        }
        count
    }
}

