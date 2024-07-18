use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let n = nums2.len();
        let mut map = HashMap::new();
        for i in 0..n {
            while !stack.is_empty() && nums2[*stack.last().unwrap()] < nums2[i] {
                map.insert(nums2[stack.pop().unwrap()], nums2[i]);
            }
            stack.push(i);
        }
        nums1.iter().map(|n| *map.get(n).unwrap_or(&-1)).collect::<Vec<_>>()
    }
}
