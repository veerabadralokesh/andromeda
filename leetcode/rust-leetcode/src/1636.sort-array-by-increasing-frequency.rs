use std::collections::HashMap;
use std::cmp::Reverse;
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for &n in &nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut vc = Vec::new();
        for (v, c) in map.into_iter() {
            vc.push((c, Reverse(v)));
        }
        vc.sort();
        // vc.reverse();
        let mut ans = Vec::with_capacity(nums.len());
        for (c, v) in vc.into_iter() {
            for _ in 0..c {
                ans.push(v.0);
            }
        }
        ans
    }
}
