impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let (mut left, mut ans) = (0, 1);
        for right in 1..nums.len() {
            if nums[right] == nums[right-1] {
                left = right;
            }
            ans += (right - left + 1);
        }
        ans as i64
    }
}

