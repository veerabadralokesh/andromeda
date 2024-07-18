impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut prefix_xor = nums.to_vec();
        let mut ans = vec![0; nums.len()];
        let max_xor = (1 << maximum_bit) - 1;
        ans[0] = nums[0] ^ max_xor;
        for i in 1..nums.len() {
            prefix_xor[i] ^= prefix_xor[i-1];
            ans[i] = prefix_xor[i] ^ max_xor;
        }
        ans.reverse();
        ans
    }
}

