impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        (nums2.iter().sum::<i32>() - nums1.iter().sum::<i32>())/(nums1.len() as i32)
    }
}

