impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ans:i32 = 0;
        let (mut l, mut r) = (0usize, nums.len()-1);
        while l < r {
            let s = nums[l] + nums[r];
            if s == k {
                l += 1;
                r -= 1;
                ans += 1;
            } else if s < k {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ans
    }
}

use std::collections::HashMap;

impl Solution2 {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut map:HashMap<i32,i32> = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut ans:i32 = 0;
        for key in map.clone().keys().copied() {
            let c1 = map.get(&key).unwrap().clone();
            if key == k - key {
                ans += (c1/2);
                map.insert(key, c1 - 2 * (c1/2));
            }
            else if c1 > 0 && map.contains_key(&(k-key)){
                let c2 = map.get(&(k-key)).unwrap().clone();
                let mut common = 0;
                if c1 > c2 {common = c2.clone()}
                else {common = c1.clone()}
                ans += common;
                {
                    map.insert(key, c1-common);
                    map.insert(k-key, c2-common);
                }
            }
        }
        ans
    }
}