impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut max = 0;
        let l = nums.len();
        let mut max = nums[0] * nums[1] * nums[l-1];
        max = max.max(nums[l-1] * nums[l-2] * nums[l-3]);
        max
    }
}

