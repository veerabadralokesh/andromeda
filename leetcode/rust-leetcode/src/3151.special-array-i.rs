impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let (mut parity, mut pparity) = (nums[0]&1, 0);
        pparity = parity;
        for &n in nums.iter().skip(1) {
            parity = n & 1;
            if parity == pparity {
                return false;
            }
            pparity = parity;
        }
        true
    }
}

