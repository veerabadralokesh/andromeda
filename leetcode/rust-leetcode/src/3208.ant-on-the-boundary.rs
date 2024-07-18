impl Solution {
    pub fn return_to_boundary_count(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..nums.len() {
            nums[i] += nums[i-1];
            if nums[i] == 0 {
                ans += 1;
            }
        }
        ans
    }
}
