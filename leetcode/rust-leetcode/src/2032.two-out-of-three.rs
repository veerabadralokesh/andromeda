impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut masks = [0i32; 101];
        for &n in nums1.iter() {
            masks[n as usize] = 1;
        }
        for &n in nums2.iter() {
            masks[n as usize] |= (1 << 1);
        }
        for &n in nums3.iter() {
            masks[n as usize] |= (1 << 2);
        }
        (1..101).filter(|&n| masks[n as usize].count_ones() > 1).collect()
    }
}

