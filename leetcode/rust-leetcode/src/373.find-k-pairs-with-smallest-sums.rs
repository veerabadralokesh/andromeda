use std::collections::BinaryHeap;
// use std::cmp::Reverse;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap:BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();
        let mut ksmallest = Vec::with_capacity(k as usize);
        let mut count = 0i32;

        for &i in &nums1 {
            for &j in &nums2 {
                if count == k {
                    let x = heap.peek().unwrap().0;
                    if i+j < x {
                        heap.pop();
                        heap.push((i+j, vec![i, j]));
                    } else {
                        break;
                    }
                } else {
                    heap.push((i + j, vec![i, j]));
                    count += 1;
                }
            }
        }

        while let Some((x, y)) = heap.pop() {
            ksmallest.push(y);
        }
        ksmallest
    }
}

