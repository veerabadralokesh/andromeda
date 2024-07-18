use std::collections::HashMap;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &n in nums.iter() {
            if n & 1 == 0 {
                *map.entry(n).or_insert(0) += 1;
            }
        }
        if map.len() == 0 {
            return -1;
        }
        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_by_key(|&e| (-e.1, e.0));
        vec[0].0
    }
}

/* */

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max_count = 0;
        let mut count;
        let mut ans = -1;
        for &n in nums.iter() {
            if n & 1 == 0 {
                count = *map.entry(n).and_modify(|c| *c += 1).or_insert(1);
                if count > max_count {
                    max_count = count;
                    ans = n;
                } else if count == max_count {
                    ans = ans.min(n);
                }
            }
        }
        ans
    }
}

