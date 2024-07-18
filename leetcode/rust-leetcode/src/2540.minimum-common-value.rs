impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i1, mut i2) = (0usize, 0usize);
        while i1 < nums1.len() && i2 < nums2.len() {
            if nums1[i1] == nums2[i2] {
                return nums1[i1];
            } else if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        -1i32
    }
}
