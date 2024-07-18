impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            ans.insert(index[i] as usize, nums[i]);
        }
        ans
    }
}