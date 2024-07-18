impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = [0; 1001];
        let mut ans:Vec<i32> = Vec::with_capacity(2000);
        for n in nums1 {
            nums[n as usize] += 1;
        }
        for n in nums2 {
            if nums[n as usize] > 0 {
                ans.push(n);
                nums[n as usize] = 0;
            }
        }
        ans
    }
}

/* */

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1 = nums1.into_iter().fold(HashSet::new(), |mut acc, n| {acc.insert(n); acc});
        nums2.into_iter().filter(|n| set1.contains(n)).fold(HashSet::new(), |mut acc, n| {acc.insert(n); acc}).into_iter().collect()
    }
}

