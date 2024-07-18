use std::collections::{HashSet,HashMap};
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut unique_set:HashSet<Vec<i32>> = HashSet::new();
        let mut map = HashMap::new();
        let n = nums.len();
        let mut duplicates = HashSet::new();
        for i in 0..n-1 {
            if !duplicates.contains(&nums[i]) {
                duplicates.insert(nums[i]);
                map.clear();
                for j in i+1..n {
                    match map.get(&nums[j]) {
                        None => {
                            map.insert(-(nums[i]+nums[j]), nums[j]);
                        },
                        Some(k) => {
                            // println!("{}, {}, {}", nums[i], nums[j], *k);
                            if nums[i] + nums[j] + *k == 0 {
                                let mut v = vec![nums[i], nums[j], *k];
                                v.sort();
                                unique_set.insert(v);
                            }
                        }
                    }
                }
            }
        }
        unique_set.into_iter().collect()
    }
}

/* */

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>>= Vec::new();
        if nums.len() < 3 { return output }
        nums.sort();
        let mut i = 0;
        while i < nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    output.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
            i += 1;
        }
        output
    }
}
