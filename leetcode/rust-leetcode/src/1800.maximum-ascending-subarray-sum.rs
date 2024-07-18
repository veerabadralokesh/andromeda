impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let (mut sum, mut max_sum) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            if nums[i] <= nums[i-1] {
                sum = 0;
            }
            sum += nums[i];
            max_sum = sum.max(max_sum);
        }
        max_sum
    }
}

