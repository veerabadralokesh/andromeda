impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in (0..nums.len()).step_by(2) {
            for _ in 0..nums[i] {
                ans.push(nums[i+1]);
            }
        }
        ans
    }
}

