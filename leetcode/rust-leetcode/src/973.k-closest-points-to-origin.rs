impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_by_key(|p| p[0].pow(2) + p[1].pow(2));
        points[..k as usize].to_vec()
    }
}

/* */

use std::collections::BinaryHeap;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for p in points {
            let (x, y) = (p[0],p[1]);
            heap.push((-(x*x+y*y), p));
        }
        let mut ans:Vec<Vec<i32>> = Vec::with_capacity(k);
        for _ in 0..k {
            ans.push(heap.pop().unwrap().1);
        }
        ans
    }
}

