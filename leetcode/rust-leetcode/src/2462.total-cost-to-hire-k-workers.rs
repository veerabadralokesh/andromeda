use std::collections::{BinaryHeap,HashSet};
use std::cmp::Reverse;
impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let costs = costs.iter().map(|&c| c as i64).collect::<Vec<_>>();
        let candidates = candidates as usize;
        let l = costs.len();
        // println!("{}", costs.len());
        // if l == candidates {
        //     return costs.iter().sum();
        // }
        let mut heap = BinaryHeap::new();
        let (mut left, mut right) = (0, l-1);
        let mut set = HashSet::new();
        for i in 0..candidates {
            // println!("{} {}", costs[left], left);
            // println!("{} {}", costs[right], right);
            if !set.contains(&left) {
                heap.push((-costs[left], Reverse(left)));
                set.insert(left);
                left += 1;
            }
            if !set.contains(&right) {
                heap.push((-costs[right], Reverse(right)));
                set.insert(right);
                right -= 1;
            }
        }
        let mut cost = 0;
        let mut count = 0;
        while let Some((c, i)) = heap.pop() {
            // println!("{} {}", -c, i.0);
            cost += -c as i64;
            count += 1;
            if count == k {
                break;
            }
            if i.0 < left && !set.contains(&left) {
                heap.push((-costs[left], Reverse(left)));
                set.insert(left);
                left += 1;
            }
            if i.0 > right && !set.contains(&right) {
                heap.push((-costs[right], Reverse(right)));
                set.insert(right);
                right -= 1;
            }
        }
        cost
    }
}

