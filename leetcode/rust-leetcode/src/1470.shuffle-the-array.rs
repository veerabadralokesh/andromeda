impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut nums2 = vec![0; nums.len()];
        for i in 0..n {
            nums2[2*i] = nums[i];
            nums2[2*i+1] = nums[n+i];
        }
        nums2
    }
}

