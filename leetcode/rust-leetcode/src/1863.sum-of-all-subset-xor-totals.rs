impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &Vec<i32>, i: usize, xor: i32) -> i32 {
            if i == nums.len() {
                xor
            } else {
                backtrack(nums, i + 1, nums[i] ^ xor) + backtrack(nums, i + 1, xor)
            }
        };
        backtrack(&nums, 0, 0)
    }
}

/* */

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0 .. nums.len() {
            res |= nums[i];
        }
        return res << (nums.len() - 1);
    }
}
