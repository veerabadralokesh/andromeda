impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans:i32 = nums[0];
        for n in nums.iter().skip(1) {
            ans ^= n;
        }
        ans
    }
}

impl Solution2 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut xor_chain: i32 = 0i32;
        for i in nums {
            xor_chain = xor_chain ^ i;
        }
        xor_chain
    }
}
