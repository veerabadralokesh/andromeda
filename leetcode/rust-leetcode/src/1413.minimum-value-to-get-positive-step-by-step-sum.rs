impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut sum = 1;
        for i in 0..nums.len() {
            sum += nums[i];
            if sum < 1 {
                ans += (1-sum);
                sum = 1;
            }
        }
        ans
    }
}
