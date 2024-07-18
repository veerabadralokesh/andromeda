use std::collections::HashMap;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut nums2:Vec<i32> = Vec::with_capacity(nums.len());
        let mut map:HashMap<i32, i32> = HashMap::new();
        let mut uniq:Vec<i32> = Vec::new();
        for n in nums.iter() {
            let count = map.entry(*n).or_insert(0);
            if (*count == 0) {
                uniq.push(*n);
            }
            *count += 1;
        }
        uniq.sort();
        let mut count:i32 = 0;
        let mut prev_count:i32 = 0;
        for n in uniq.iter() {
            count += map.get(&n).copied().unwrap_or(0);
            map.insert(*n, prev_count);
            prev_count = count;
        }
        for n in nums.iter() {
            nums2.push(map.get(n).copied().unwrap_or(0));
        }
        nums2
    }
}

impl Solution2 {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&n| {
            nums.iter().filter(|&&m| m < n).count() as i32
        }).collect()
    }
}
