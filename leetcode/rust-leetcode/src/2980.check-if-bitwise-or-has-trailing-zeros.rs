impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for &n in nums.iter() {
            if n & 1 == 0 {
                count += 1;
                if count > 1 {
                    return true;
                }
            }
        }
        false
    }
}

