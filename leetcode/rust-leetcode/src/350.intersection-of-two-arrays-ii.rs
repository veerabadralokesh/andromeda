use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for n in nums1 {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut nums:Vec<i32> = Vec::new();
        for n in nums2 {
            let mut count = map.entry(n).or_insert(0);
            if *count > 0 {
                nums.push(n);
                *count -= 1;
            }
        }
        nums
    }
}

impl Solution2 {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut result = Vec::new();
        
        let mut ptr1: usize = 0;
        let mut ptr2: usize = 0;
        
        while ptr1 < nums1.len() && ptr2 < nums2.len() {
            match nums1[ptr1].cmp(&nums2[ptr2]) {
                std::cmp::Ordering::Less => {
                    ptr1 += 1;
                },
                std::cmp::Ordering::Equal => {
                    result.push(nums1[ptr1]);
                    ptr1 += 1;
                    ptr2 += 1;
                },
                std::cmp::Ordering::Greater => {
                    ptr2 += 1;
                }
            }
        }
        return result;
    }
}