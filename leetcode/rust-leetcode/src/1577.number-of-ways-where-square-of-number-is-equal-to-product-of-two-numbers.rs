use std::collections::HashMap;
impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1 = nums1.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let nums2 = nums2.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let mut products = HashMap::new();
        for j in 0..nums1.len() {
            for k in j+1..nums1.len() {
                *products.entry(nums1[j]*nums1[k]).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for &n in nums2.iter() {
            ans += *products.get(&(n*n)).unwrap_or(&0);
        }
        products.clear();
        for j in 0..nums2.len() {
            for k in j+1..nums2.len() {
                *products.entry(nums2[j]*nums2[k]).or_insert(0) += 1;
            }
        }
        for &n in nums1.iter() {
            ans += *products.get(&(n*n)).unwrap_or(&0);
        }
        ans
    }
}

