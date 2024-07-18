use std::cmp::{max,min};
impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        if n1 > n2 {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let (mut l, mut r) = (0, n1);
        while l <= r {
            let p1 = l + (r - l) / 2;
            let p2 = (n1 + n2 + 1) / 2 - p1;

            let max_l1 = if p1 == 0 {i32::MIN} else {nums1[p1 as usize - 1]};
            let max_l2 = if p2 == 0 {i32::MIN} else {nums2[p2 as usize - 1]};
            let min_r1 = if p1 == n1 {i32::MAX} else {nums1[p1 as usize]};
            let min_r2 = if p2 == n2 {i32::MAX} else {nums2[p2 as usize]};
            if max_l1 <= min_r2 && max_l2 <= min_r1 {
                if (n1 + n2) & 1 == 0 {
                    return (max(max_l1, max_l2) + min(min_r1, min_r2)) as f64 * 0.5
                } else {
                    return max(max_l1, max_l2) as _
                }
            } else if max_l1 > min_r2 {
                r = p1 - 1;
            } else {
                l = p1 + 1;
            }
        }
        0.0
    }
}

