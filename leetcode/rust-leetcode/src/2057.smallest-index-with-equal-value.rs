impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, &n) in nums.iter().enumerate() {
            if nums[i] == (i as i32) % 10 {
                return i as i32;
            }
        }
        -1
    }
}

