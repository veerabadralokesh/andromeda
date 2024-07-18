use std::collections::*;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut ans = 0;
        for (i, &n) in nums.iter().enumerate() {
            *counts.entry(n).or_insert(0) += 1;
        }
        let mut nums = counts.keys().collect::<HashSet<_>>();
        for &n in nums.iter() {
            if nums.contains(&(n-1)) {
                ans = ans.max(
                    *counts.get(&n).unwrap() + *counts.get(&(n-1)).unwrap()
                );
            }
        }
        ans as _
    }
}

